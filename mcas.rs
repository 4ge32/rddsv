use crate::main::*;

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

fn action_p_unlock(_prop: Prop, p: &mut SharedVars, _q: &SharedVars) {
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

fn action_q_unlock(_prop: Prop, q: &mut SharedVars, _p: &SharedVars) {
    q.x = 0;
}

fn m_cas_p_def() -> Process {
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

    return p;
}

fn m_cas_q_def() -> Process {
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

    return q;
}

fn m_cas_def() -> Vec<Process> {
    let p = m_cas_p_def();
    let q = m_cas_q_def();
    let process = vec![p, q];

    process
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn run() {
    /* visualize each process */
    let process = m_cas_def();
    vis_process("P_cas", &process[0]);
    vis_process("Q_cas", &process[1]);

    let r: SharedVars = Default::default();
    let s = Rc::new(RefCell::new(State::new(r, 2)));
    let lts = concurrent_composition(process, s);
    vis_lts(&lts);
}

mod test {
    use super::*;
    use file_diff::diff_files;
    use std::fs::*;

    #[test]
    fn vis_p_process() {
        let p = m_cas_p_def();
        vis_process("P_cas", &p);

        let mut file1 = match File::open("./P_cas.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("./tests/m_cas_P.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        assert!(diff_files(&mut file1, &mut file2), "Not much P dot diff");

        std::fs::remove_file("P_cas.dot").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    #[test]
    fn vis_q_process() {
        let p = m_cas_q_def();
        vis_process("Q_cas", &p);

        let mut file1 = match File::open("./Q_cas.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("./tests/m_cas_Q.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        assert!(diff_files(&mut file1, &mut file2), "Not much Q dot diff");

        std::fs::remove_file("Q_cas.dot").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    #[test]
    fn process_create() {
        let process = m_cas_def();
        let p = &process[0].name;
        let q = &process[1].name;
        assert_eq!("P", p);
        assert_eq!("Q", q);
    }
}
