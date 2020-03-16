use rddsv::lts::*;
use rddsv::process::*;

/* User definition of guard and action */
fn action_p_read(a: &mut SharedVars, b: &SharedVars) {
    a.t1 = b.x;
}

fn action_p_inc(a: &mut SharedVars, b: &SharedVars) {
    a.t1 = b.t1 + 1;
}

fn action_p_write(a: &mut SharedVars, b: &SharedVars) {
    a.x = b.t1;
}

fn m_inc2_p_def() -> Process<SharedVars> {
    /* Create Process "P" */
    let p_read = ProcessTrans::new("read", 1, guard_true, action_p_read);
    let p_inc = ProcessTrans::new("inc", 2, guard_true, action_p_inc);
    let p_write = ProcessTrans::new("write", 3, guard_true, action_p_write);
    /*TODO: Add stop state */

    let p0 = ExecUnit::new(0, vec![p_read]);
    let p1 = ExecUnit::new(1, vec![p_inc]);
    let p2 = ExecUnit::new(2, vec![p_write]);
    let p3 = ExecUnit::new(3, vec![]);

    let p_trans = vec![p0, p1, p2, p3];
    let p = Process::new("P", p_trans);

    return p;
}

fn action_q_read(a: &mut SharedVars, b: &SharedVars) {
    a.t2 = b.x;
}

fn action_q_inc(a: &mut SharedVars, b: &SharedVars) {
    a.t2 = b.t2 + 1;
}

fn action_q_write(a: &mut SharedVars, b: &SharedVars) {
    a.x = b.t2;
}

fn m_inc2_q_def() -> Process<SharedVars> {
    /* Create Process "Q" */
    let q_read = ProcessTrans::new("read", 1, guard_true, action_q_read);
    let q_inc = ProcessTrans::new("inc", 2, guard_true, action_q_inc);
    let q_write = ProcessTrans::new("write", 3, guard_true, action_q_write);

    let q0 = ExecUnit::new(0, vec![q_read]);
    let q1 = ExecUnit::new(1, vec![q_inc]);
    let q2 = ExecUnit::new(2, vec![q_write]);
    let q3 = ExecUnit::new(3, vec![]);

    let q_trans = vec![q0, q1, q2, q3];
    let q = Process::new("Q", q_trans);

    return q;
}

fn m_inc2_def() -> Vec<Process<SharedVars>> {
    let p = m_inc2_p_def();
    let q = m_inc2_q_def();
    let process = vec![p, q];

    process
}

pub fn main() {
    /* visualize each process */
    let process = m_inc2_def();
    process[0].visualize("res/m_inc2_P.dot");
    process[1].visualize("res/m_inc2_Q.dot");

    let r: SharedVars = Default::default();
    let s = State::new(r);
    let lts = concurrent_composition(process, s);
    lts.visualize("res/m_inc2.dot");
}

#[cfg(test)]
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
    }
}
