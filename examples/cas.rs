use rddsv::lts::*;
use rddsv::process::*;
use std::fmt;

#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SharedVars {
    pub x: i32,  // shared variables between P and Q.
    pub t1: i32, // P's local variables.
    pub t2: i32, // Q's local variables.
}

impl fmt::Display for SharedVars {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "x={} t1={} t2={}", self.x, self.t1, self.t2)
    }
}

#[allow(dead_code)]
pub fn guard_true(_p: SharedVars) -> bool {
    true
}

#[allow(dead_code)]
pub fn action_nop(_q: &mut SharedVars, _p: &SharedVars) {}

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

pub fn main() {
    /* visualize each process */
    let process = m_cas_def();
    process[0].visualize("res/m_cas_P.dot");

    let r: SharedVars = Default::default();
    let s = State::new(r);
    let lts = concurrent_composition(process, s);
    lts.visualize("res/m_cas.dot");
}

#[cfg(test)]
mod test {
    use super::*;
    use file_diff::diff_files;
    use std::fs::*;

    #[test]
    fn vis_process() {
        let process = m_cas_def();
        process[0].visualize("res/test_m_cas_P.dot");

        let mut file1 = match File::open("./res/test_m_cas_P.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("./ref/m_cas_P.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        assert!(diff_files(&mut file1, &mut file2), "They are different.");

        std::fs::remove_file("res/test_m_cas_P.dot").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    #[test]
    fn vis_m_cas() {
        let process = m_cas_def();
        let r: SharedVars = Default::default();
        let s = State::new(r);
        let lts = concurrent_composition(process, s);
        lts.visualize("res/test_m_cas.dot");

        let mut file1 = match File::open("./res/test_m_cas.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut file2 = match File::open("./ref/m_cas.dot") {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        assert!(diff_files(&mut file1, &mut file2), "They are different.");

        std::fs::remove_file("res/test_m_cas.dot").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
}
