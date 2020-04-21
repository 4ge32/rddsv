use crate::process::*;
use indexmap::IndexMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::io::{BufWriter, Write};

#[derive(Default, std::fmt::Debug, Clone, PartialEq, Eq, Hash)]
pub struct State<T> {
    pub shared_vars: T,
    pub locations: Vec<Location>,
    pub deadlock: bool,
}

impl<T: std::fmt::Debug + Clone + Hash + Eq> State<T> {
    pub fn new(r: T) -> State<T> {
        State {
            shared_vars: r,
            locations: vec![Location::new(0), Location::new(0)],
            deadlock: false,
        }
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> fmt::Display for State<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.shared_vars)
    }
}

pub type StateId = usize;

#[derive(Clone, PartialEq, Eq)]
pub struct Trans<T> {
    pub state: State<T>,
    pub dst: Vec<Option<(Label, Location)>>,
}

impl<T: Clone + Hash + Eq> Trans<T> {
    pub fn new(s: &State<T>, v: Option<(Label, Location)>) -> Trans<T> {
        Trans {
            state: s.clone(),
            dst: vec![v],
        }
    }
}

#[derive(Clone)]
pub struct CompTrans {
    pub label: String,
    pub before: StateId,
    pub after: StateId,
    pub on_deadlock: bool,
}

impl CompTrans {
    pub fn new(label: String, b: StateId, a: StateId) -> CompTrans {
        CompTrans {
            label: label,
            before: b,
            after: a,
            on_deadlock: false,
        }
    }
}

#[derive(Clone)]
pub struct Lts<T> {
    hat: IndexMap<State<T>, StateId>,
    dead: IndexMap<State<T>, StateId>,
    trans: Vec<CompTrans>,
}

impl<T: std::fmt::Display + Clone + Eq + Hash> Lts<T> {
    pub fn new() -> Lts<T> {
        Lts {
            hat: IndexMap::new(),
            dead: IndexMap::new(),
            trans: Vec::new(),
        }
    }

    fn make_states(&self) -> Vec<StateId> {
        let mut v = Vec::new();
        for e in self.trans.iter() {
            v.push(e.after);
        }
        v
    }

    pub fn detect_deadlock(&self) -> (bool, Vec<StateId>) {
        let mut deadlock = true;
        let v = self.make_states();
        let ret: Vec<StateId> = v
            .clone()
            .into_iter()
            .filter(|x| {
                for e in self.trans.iter() {
                    if e.before == *x {
                        return false;
                    }
                }
                true
            })
            .collect();
        if ret.len() == 0 {
            deadlock = false;
        }
        (deadlock, ret)
    }

    pub fn mark_state(&mut self, deadlock: Vec<StateId>) {
        let can = self
            .hat
            .clone()
            .iter()
            .filter(|v| {
                for e in &deadlock {
                    if *e == *v.1 {
                        return true;
                    }
                }
                false
            })
            .map(|(k, v)| {
                let mut kc = k.clone();
                kc.deadlock = true;
                (kc, *v)
            })
            .collect();
        self.dead = can;
    }

    pub fn mark_path(&mut self, deadlock: Vec<StateId>) {
        let mut can: Vec<usize> = Vec::new();
        for (i, t) in self.trans.iter().enumerate() {
            for d in deadlock.iter() {
                if t.after == *d {
                    can.push(i);
                    break;
                }
            }
        }

        let mut deadlock = Vec::new();
        let mut suc = true;
        for c in can {
            let mut e = self.trans.remove(c);
            e.on_deadlock = true;
            deadlock.push(e.before);
            self.trans.insert(c, e);
            if c == 0 {
                suc = false;
            }
        }

        if suc == true {
            self.mark_path(deadlock);
        }
    }

    /* get a lts as adjacency-list representation */
    pub fn get_ali(&self) -> Vec<Vec<StateId>> {
        let mut ret: Vec<Vec<StateId>> = vec![vec![]];
        for e in &self.trans {
            ret[e.before].push(e.after);
        }

        for _e in &ret {
            //println!("{}", );
        }
        ret
    }

    pub fn visualize(&self, path: &str) {
        let mut f = BufWriter::new(fs::File::create(path).unwrap());
        writeln!(f, "digraph {{").unwrap();
        for h in self.hat.iter() {
            write!(
                f,
                "{} [label=\"{}\\nP{} Q{} \\n{}\"",
                h.1, h.1, h.0.locations[0], h.0.locations[1], h.0.shared_vars,
            )
            .unwrap();
            if *h.1 == 0 {
                writeln!(f, "color=cyan, style=filled];").unwrap();
            } else {
                let mut deadlock = false;
                for d in self.dead.clone().iter() {
                    if d.1 == h.1 {
                        deadlock = true;
                        writeln!(f, "color=pink, style=filled];").unwrap();
                        break;
                    }
                }
                if deadlock == false {
                    writeln!(f, "];").unwrap();
                }
            }
        }
        for v in self.trans.iter() {
            write!(f, "{} -> {} [label=\"{}\"", v.before, v.after, v.label).unwrap();
            if v.on_deadlock {
                writeln!(f, "color=red,fontcolor=red,weight=2,penwidth=2];").unwrap();
            } else {
                writeln!(f, "];").unwrap();
            }
        }
        writeln!(f, "}}").unwrap();
    }
}

pub fn concurrent_composition<T: std::fmt::Display + Clone + Copy + Eq + Hash>(
    process: Vec<Process<T>>,
    s0: State<T>,
) -> Lts<T> {
    let mut lts = Lts::new();
    let mut que: VecDeque<Trans<T>> = VecDeque::new();

    let trans0 = Trans::new(&s0, None);
    que.push_back(trans0);
    lts.hat.insert(s0, 0);

    loop {
        if let Some(trans) = que.pop_front() {
            let s = trans.state;
            /* for each process */
            for i in 0..process.len() {
                let loc = s.locations[i].clone();
                let pp = &process[i].v[loc.to_usize()];
                for p in &pp.transs {
                    if (p.guard)(process[i].prop, s.shared_vars) {
                        let mut t = s.clone();
                        t.locations[i] = p.dst;
                        (p.action)(process[i].prop, &mut t.shared_vars, &s.shared_vars);
                        let before_id = *lts.hat.get(&s).unwrap();
                        let mut after_id = lts.hat.len();
                        match lts.hat.get(&t) {
                            None => {
                                let trans = Trans::new(&t, Some((process[i].label.clone(), p.dst)));
                                lts.hat.insert(t.clone(), after_id);
                                que.push_back(trans);
                            }
                            Some(exist) => {
                                after_id = *exist;
                            }
                        }
                        let l = format!("{}.{}", process[i].label, p.label.clone());
                        lts.trans.push(CompTrans::new(l, before_id, after_id));
                    }
                }
            }
        } else {
            break;
        }
    }
    let on_deadlock = lts.detect_deadlock();
    if on_deadlock.0 {
        lts.mark_path(on_deadlock.1.clone());
        lts.mark_state(on_deadlock.1);
    }
    //lts.get_ali();
    lts
}
