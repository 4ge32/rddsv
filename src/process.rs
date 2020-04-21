use std::fs;
use std::fmt;
use std::io::{BufWriter, Write};

pub type Prop = i32;
pub type Guard<T> = fn(Prop, T) -> bool;
pub type Action<T> = fn(Prop, &mut T, &T);

#[allow(dead_code)]
pub fn guard_true<T: Clone + Eq>(_prop: Prop, _p: T) -> bool {
    true
}

#[allow(dead_code)]
pub fn action_nop<T: Clone + Eq>(_prop: Prop, _q: &mut T, _p: &T) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(usize);

impl Location {
    pub fn new(s: usize) -> Self {
        Location(s)
    }
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label(pub String);

impl Label {
    pub fn new(s: &str) -> Self {
        Label(s.to_string())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct ProcessTrans<T> {
    pub label: Label,
    pub dst: Location,
    pub guard: Guard<T>,
    pub action: Action<T>,
}

impl<T: Clone + Eq> ProcessTrans<T> {
    pub fn new(name: &str, dst: usize, guard: Guard<T>, action: Action<T>) -> ProcessTrans<T> {
        ProcessTrans {
            label: Label::new(name),
            dst: Location::new(dst),
            guard: guard,
            action: action,
        }
    }
}

#[derive(Clone)]
pub struct ExecUnit<T> {
    pub src: Location,
    pub transs: Vec<ProcessTrans<T>>,
}

impl<T: Clone + Eq> ExecUnit<T> {
    pub fn new(src: usize, trans: Vec<ProcessTrans<T>>) -> Self {
        ExecUnit {
            src: Location::new(src),
            transs: trans,
        }
    }
}

#[derive(Clone)]
pub struct Process<T> {
    pub label: Label,
    pub v: Vec<ExecUnit<T>>,
    pub prop: Prop,
}

impl<T: Clone> Process<T> {
    pub fn new(label: &str, v: Vec<ExecUnit<T>>) -> Process<T> {
        Process {
            label: Label::new(label),
            v: v,
            prop: 0,
        }
    }

    pub fn visualize(&self, path: &str) {
        let mut f = BufWriter::new(fs::File::create(path).unwrap());
        writeln!(f, "digraph {{").unwrap();
        for (i, e) in self.v.iter().enumerate() {
            writeln!(f, "{} [label=\"{}{}\"];", i, self.label, e.src).unwrap();
        }
        for (i, e) in self.v.iter().enumerate() {
            for pt in &e.transs {
                writeln!(f, "{} -> {} [label=\"{}\"]", i, pt.dst, pt.label).unwrap();
            }
        }
        writeln!(f, "}}").unwrap();
    }
}
