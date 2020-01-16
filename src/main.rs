type Prop = i32;
type Guard = fn(prop: Prop, p: &SharedVars) -> bool;
type Action = fn(prop: Prop, q: &mut SharedVars, p: &SharedVars);

#[derive(Default)]
pub struct SharedVars {
    x: i32,  // shared variables betwenn P and Q.
    t1: i32, // P's local variables.
    t2: i32, // Q's local variables.
}

pub struct ProcessTrans {
    label: String,
    target: u32,
    guard: Guard,
    action: Action,
}

impl ProcessTrans {
    pub fn new(name: impl Into<String>, target: u32, guard: Guard, action: Action) -> ProcessTrans {
        ProcessTrans {
            label: name.into(),
            target: target,
            guard: guard,
            action: action,
        }
    }
}

pub struct Process {
    name: String,
    prop: Prop,
    trans: Vec<Vec<ProcessTrans>>,
}

impl Process {
    pub fn new(name: impl Into<String>, prop: Prop, trans: Vec<Vec<ProcessTrans>>) -> Process {
        Process {
            name: name.into(),
            prop: prop,
            trans: trans,
        }
    }
}

#[allow(dead_code)]
fn guard_true(_prop: Prop, _p: &SharedVars) -> bool {
    true
}

#[allow(dead_code)]
fn action_nop(_prop: Prop, _q: &mut SharedVars, _p: &SharedVars) {}

/* User definition of guard and action */
fn action_p_cas(_prop: Prop, p: &mut SharedVars, q: &SharedVars) {
    p.x = 1;
    p.t1 = q.x;
}

fn guard_p_retry(_prop: Prop, p: &SharedVars) -> bool {
    p.t1 == 1
}

fn action_p_retry(_prop: Prop, p: &mut SharedVars, _q: &SharedVars) {
    p.t1 = 0;
}

fn guard_p_begin(_prop: Prop, p: &SharedVars) -> bool {
    p.t1 == 0
}

fn action_p_unlock(_prop: Prop, p: &mut SharedVars, q: &SharedVars) {
    p.x = 0;
}

fn action_q_cas(_prop: Prop, q: &mut SharedVars, p: &SharedVars) {
    q.x = 1;
    q.t2 = p.x;
}

fn guard_q_retry(_prop: Prop, q: &SharedVars) -> bool {
    q.t2 == 1
}

fn action_q_retry(_prop: Prop, q: &mut SharedVars, _p: &SharedVars) {
    q.t2 = 0;
}

fn guard_q_begin(_prop: Prop, q: &SharedVars) -> bool {
    q.t2 == 0
}

fn action_q_unlock(_prop: Prop, q: &mut SharedVars, p: &SharedVars) {
    q.x = 0;
}
fn main() {
    /* Create Process "P" */
    let p_cas = ProcessTrans::new("CAS", 1, guard_true, action_p_cas);
    let p_retry = ProcessTrans::new("retry", 0, guard_p_retry, action_p_retry);
    let p_begin = ProcessTrans::new("begin", 2, guard_p_begin, action_nop);
    let p_end = ProcessTrans::new("end", 3, guard_true, action_p_unlock);
    let p_unlock = ProcessTrans::new("unlock", 0, guard_true, action_p_unlock);

    let p0 = vec![p_cas];
    let p1 = vec![p_retry, p_begin];
    let p2 = vec![p_end];
    let p3 = vec![p_unlock];

    let p_trans = vec![p0, p1, p2, p3];
    let p = Process::new("P", 1, p_trans);

    /* Create Process "Q" */
    let q_cas = ProcessTrans::new("CAS", 1, guard_true, action_q_cas);
    let q_retry = ProcessTrans::new("retry", 0, guard_q_retry, action_q_retry);
    let q_begin = ProcessTrans::new("begin", 2, guard_q_begin, action_nop);
    let q_end = ProcessTrans::new("end", 3, guard_true, action_q_unlock);
    let q_unlock = ProcessTrans::new("unlock", 0, guard_true, action_q_unlock);

    let q0 = vec![q_cas];
    let q1 = vec![q_retry, q_begin];
    let q2 = vec![q_end];
    let q3 = vec![q_unlock];

    let q_trans = vec![q0, q1, q2, q3];
    let q = Process::new("Q", 1, q_trans);

    /* visualize each process */
    //vis_process(&p);
    //vis_process(&q);
}

/* ddsv */
pub fn vis_process(process: &Process) {
    print_prologue();

    let mut i = 0;
    for _p in &process.trans {
        println!("{} [label=\"{}{}\"];", i, process.name, i);
        i += 1;
    }

    i = 0;
    for p in &process.trans {
        for q in &(*p) {
            println!("{} -> {} [label=\"{}\"];", i, q.target, q.label);
        }
        i += 1;
    }

    print_epilogue();
}

fn print_prologue() {
    println!("digraph {{");
}

fn print_epilogue() {
    println!("}}");
}

mod tests {
    use super::*;

    #[test]
    fn new_process_trans() {
        let p0 = ProcessTrans::new("CAS", 1, guard_true, action_nop);
        assert_eq!(p0.label, "CAS");
        assert_eq!(p0.target, 1);
    }
    #[test]
    fn default_shared_vars() {
        let sv: SharedVars = Default::default();
        assert_eq!(sv.t1, 0);
        assert_eq!(sv.t2, 0);
        assert_eq!(sv.x, 0);
    }
    #[test]
    fn __conf_ret_false() {
        let p0 = ProcessTrans::new("CAS", 1, guard_p_retry, action_nop);
        let sv: SharedVars = Default::default();
        assert_eq!((p0.guard)(0, &sv), false);
    }
    #[test]
    fn __conf_ret_true() {
        let p0 = ProcessTrans::new("CAS", 1, guard_p_retry, action_nop);
        let mut sv: SharedVars = Default::default();
        sv.t1 = 1;
        assert_eq!((p0.guard)(0, &sv), true);
    }
}
