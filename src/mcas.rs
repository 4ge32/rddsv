use crate::process::*;
use crate::lts::*;

/* User definition of guard and action */
fn action_p_cas(p: &mut SharedVars, q: &SharedVars) {
    p.x = 1;
    p.t1 = q.x;
}

fn guard_p_retry(p: SharedVars) -> bool {
    p.t1 == 1
}

fn action_p_retry(p: &mut SharedVars, _q: &SharedVars) {
    p.t1 = 0;
}

fn guard_p_begin(p: SharedVars) -> bool {
    p.t1 == 0
}

fn action_p_unlock(p: &mut SharedVars, _q: &SharedVars) {
    p.x = 0;
}

fn action_q_cas(q: &mut SharedVars, p: &SharedVars) {
    q.x = 1;
    q.t2 = p.x;
}

fn guard_q_retry(q: SharedVars) -> bool {
    q.t2 == 1
}

fn action_q_retry(q: &mut SharedVars, _p: &SharedVars) {
    q.t2 = 0;
}

fn guard_q_begin(q: SharedVars) -> bool {
    q.t2 == 0
}

fn action_q_unlock(q: &mut SharedVars, _p: &SharedVars) {
    q.x = 0;
}

fn m_cas_p_def() -> Process<SharedVars> {
    /* Create Process "P" */
    let p_cas = ProcessTrans::new("CAS", 1, guard_true, action_p_cas);
    let p_retry = ProcessTrans::new("retry", 0, guard_p_retry, action_p_retry);
    let p_begin = ProcessTrans::new("begin", 2, guard_p_begin, action_nop);
    let p_end = ProcessTrans::new("end", 3, guard_true, action_nop);
    let p_unlock = ProcessTrans::new("unlock", 0, guard_true, action_p_unlock);

    let p0 = ExecUnit::new(0, vec![p_cas]);
    let p1 = ExecUnit::new(1, vec![p_retry, p_begin]);
    let p2 = ExecUnit::new(2, vec![p_end]);
    let p3 = ExecUnit::new(3, vec![p_unlock]);

    let p = vec![p0, p1, p2, p3];
    let ret = Process::new("P", p);

    ret
}

fn m_cas_q_def() -> Process<SharedVars> {
    /* Create Process "Q" */
    let q_cas = ProcessTrans::new("CAS", 1, guard_true, action_q_cas);
    let q_retry = ProcessTrans::new("retry", 0, guard_q_retry, action_q_retry);
    let q_begin = ProcessTrans::new("begin", 2, guard_q_begin, action_nop);
    let q_end = ProcessTrans::new("end", 3, guard_true, action_nop);
    let q_unlock = ProcessTrans::new("unlock", 0, guard_true, action_q_unlock);

    let q0 = ExecUnit::new(0, vec![q_cas]);
    let q1 = ExecUnit::new(1, vec![q_retry, q_begin]);
    let q2 = ExecUnit::new(2, vec![q_end]);
    let q3 = ExecUnit::new(3, vec![q_unlock]);

    let q = vec![q0, q1, q2, q3];
    let ret = Process::new("Q", q);

    ret
}

fn m_cas_def() -> Vec<Process<SharedVars>> {
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
    //vis_process("P_cas", &process[0]);
    //vis_process("Q_cas", &process[1]);

    let r: SharedVars = Default::default();
    let s = State::new(r);
    let lts = concurrent_composition(process, s);
    lts.visualize("m_cas.dot");
}

mod test {
    use super::*;
    use file_diff::diff_files;
    use std::fs::*;

    //#[test]
    //fn vis_p_process() {
    //    let p = m_cas_p_def();
    //    vis_process("P_cas", &p);

    //    let mut file1 = match File::open("./P_cas.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };
    //    let mut file2 = match File::open("./tests/m_cas_P.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };

    //    assert!(diff_files(&mut file1, &mut file2), "Not much P dot diff");

    //    std::fs::remove_file("P_cas.dot").unwrap_or_else(|why| {
    //        println!("! {:?}", why.kind());
    //    });
    //}
    //#[test]
    //fn vis_q_process() {
    //    let p = m_cas_q_def();
    //    vis_process("Q_cas", &p);

    //    let mut file1 = match File::open("./Q_cas.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };
    //    let mut file2 = match File::open("./tests/m_cas_Q.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };

    //    assert!(diff_files(&mut file1, &mut file2), "Not much Q dot diff");

    //    std::fs::remove_file("Q_cas.dot").unwrap_or_else(|why| {
    //        println!("! {:?}", why.kind());
    //    });
    //}
    //#[test]
    //fn process_create() {
    //    let process = m_cas_def();
    //    let p = &process[0].name;
    //    let q = &process[1].name;
    //    assert_eq!("P", p);
    //    assert_eq!("Q", q);
    //}
}
