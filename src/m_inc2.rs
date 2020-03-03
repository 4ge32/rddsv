use crate::ddsv::*;

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

fn action_p_read(_prop: Prop, p: &mut SharedVars, q: &SharedVars) {
    p.t1 = q.x;
}

fn action_p_inc(_prop: Prop, p: &mut SharedVars, q: &SharedVars) {
    p.x = q.t1 + 1;
}

fn action_p_write(_prop: Prop, p: &mut SharedVars, q: &SharedVars) {
    p.x = q.t1;
}

fn m_inc2_p_def() -> Process {
    /* Create Process "P" */
    let p_read = ProcessTrans::new("read", 1, guard_true, action_p_read);
    let p_inc = ProcessTrans::new("inc", 1, guard_true, action_p_inc);
    let p_write = ProcessTrans::new("inc", 1, guard_true, action_p_write);

    let p0 = vec![p_read];
    let p1 = vec![p_inc];
    let p2 = vec![p_write];

    let p_trans = vec![p0, p1, p2];
    let p = Process::new("P", 1, p_trans);

    return p;
}

fn action_q_read(_prop: Prop, q: &mut SharedVars, p: &SharedVars) {
    q.t2 = p.x;
}

fn action_q_inc(_prop: Prop, q: &mut SharedVars, p: &SharedVars) {
    q.t2 = p.t2 + 1;
}

fn action_q_write(_prop: Prop, q: &mut SharedVars, p: &SharedVars) {
    q.x = p.t2;
}

fn m_inc2_q_def() -> Process {
    /* Create Process "Q" */
    let q_read = ProcessTrans::new("read", 1, guard_true, action_q_read);
    let q_inc = ProcessTrans::new("inc", 1, guard_true, action_q_inc);
    let q_write = ProcessTrans::new("write", 1, guard_true, action_q_write);

    let q0 = vec![q_read];
    let q1 = vec![q_inc];
    let q2 = vec![q_write];

    let q_trans = vec![q0, q1, q2];
    let q = Process::new("Q", 1, q_trans);

    return q;
}

fn m_cas_def() -> Vec<Process> {
    let p = m_inc2_p_def();
    let q = m_inc2_q_def();
    let process = vec![p, q];

    process
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn m_inc() {
    /* visualize each process */
    let process = m_cas_def();
    vis_process("P_inc2", &process[0]);
    vis_process("Q_inc2", &process[1]);

    let r: SharedVars = Default::default();
    let s = Rc::new(RefCell::new(State::new(r, 2)));
    let lts = concurrent_composition(process, s);
    vis_lts(&lts);
}

mod test {
    use super::*;
    use file_diff::diff_files;
    use std::fs::*;

    //#[test]
    //fn vis_p_process() {
    //    let p = m_inc2_p_def();
    //    vis_process("P_inc2", &p);

    //    let mut file1 = match File::open("./P_inc2.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };
    //    let mut file2 = match File::open("./tests/m_inc2_P.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };

    //    assert!(diff_files(&mut file1, &mut file2), "Not much P dot diff");

    //    std::fs::remove_file("P_inc2.dot").unwrap_or_else(|why| {
    //        println!("! {:?}", why.kind());
    //    });
    //}
    //#[test]
    //fn vis_q_process() {
    //    let p = m_inc2_q_def();
    //    vis_process("Q_inc2", &p);

    //    let mut file1 = match File::open("./Q_inc2.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };
    //    let mut file2 = match File::open("./tests/m_inc2_Q.dot") {
    //        Ok(f) => f,
    //        Err(e) => panic!("{}", e),
    //    };

    //    assert!(diff_files(&mut file1, &mut file2), "Not much Q dot diff");

    //    std::fs::remove_file("Q_inc2.dot").unwrap_or_else(|why| {
    //        println!("! {:?}", why.kind());
    //    });
    //}
    #[test]
    fn process_create() {
        let process = m_cas_def();
        let p = &process[0].name;
        let q = &process[1].name;
        assert_eq!("P", p);
        assert_eq!("Q", q);
    }
}
