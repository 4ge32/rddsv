use crate::process::*;
use indexmap::IndexMap;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::fs;
use std::io::{BufWriter, Write};

#[derive(Default, std::fmt::Debug, Clone, PartialEq, Eq, Hash)]
pub struct State<T> {
    pub shared_vars: T,
    pub locations: Vec<Location>,
}

impl<T: std::fmt::Debug + Clone + Hash + Eq> State<T> {
    pub fn new(r: T) -> State<T> {
        State {
            shared_vars: r,
            locations: vec![Location::new(0), Location::new(0)],
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

pub struct Lts<T>(pub Vec<Trans<T>>);

impl<T: Clone + Hash + Eq> Lts<T> {
    pub fn new() -> Self {
        Lts(vec![])
    }
}

pub struct CompTrans {
    pub label: String,
    pub before: StateId,
    pub after: StateId,
}

impl CompTrans {
    pub fn new(label: String, b: StateId, a: StateId) -> CompTrans {
        CompTrans {
            label: label,
            before: b,
            after: a,
        }
    }
}

pub struct CompLts<T> {
    hat: IndexMap<State<T>, StateId>,
    trans: Vec<CompTrans>,
}

impl<T: std::fmt::Display + Clone + Eq + Hash> CompLts<T> {
    pub fn new() -> CompLts<T> {
        CompLts {
            hat: IndexMap::new(),
            trans: Vec::new(),
        }
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
                writeln!(f, "];").unwrap();
            }
        }
        for v in self.trans.iter() {
            writeln!(f, "{} -> {} [label=\"{}\"];", v.before, v.after, v.label).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }
}

pub fn concurrent_composition<T: std::fmt::Display + Clone + Copy + Eq + Hash>(
    process: Vec<Process<T>>,
    s0: State<T>,
) -> CompLts<T> {
    let mut lts = CompLts::new();
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
                    if (p.guard)(s.shared_vars) {
                        let mut t = s.clone();
                        t.locations[i] = p.dst;
                        (p.action)(&mut t.shared_vars, &s.shared_vars);
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
    lts
}
