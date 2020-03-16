use rddsv::lts::*;
use rddsv::process::*;
use std::fmt;

#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SharedVars {
    pub m0: i32,  // mutex0
    pub m1: i32,  // mutex1
}

impl fmt::Display for SharedVars {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "m0={} m1={}", self.m0, self.m1)
    }
}

#[allow(dead_code)]
pub fn guard_true(_p: SharedVars) -> bool {
    true
}

#[allow(dead_code)]
pub fn action_nop(_q: &mut SharedVars, _p: &SharedVars) {}

/* User definition of guard and action
 * a = after, b = before, c = current
 */
fn guard_lock0(c: SharedVars) -> bool {
    c.m0 == 0
}

fn guard_lock1(c: SharedVars) -> bool {
    c.m1 == 0
}

fn action_lock0(a: &mut SharedVars, _b: &SharedVars) {
    a.m0 = 1
}

fn action_lock1(a: &mut SharedVars, _b: &SharedVars) {
    a.m1 = 1
}

fn action_unlock0(a: &mut SharedVars, _b: &SharedVars) {
    a.m0 = 0
}

fn action_unlock1(a: &mut SharedVars, _b: &SharedVars) {
    a.m1 = 0
}

fn p_def() -> Process<SharedVars> {
    /* Create Process "P" */
    let p_lock0 = ProcessTrans::new("lock0", 1, guard_lock0, action_lock0);
    let p_lock1 = ProcessTrans::new("lock1", 2, guard_lock1, action_lock1);
    let p_unlock1 = ProcessTrans::new("unlock1", 3, guard_true, action_unlock1);
    let p_unlock0 = ProcessTrans::new("unlock0", 0, guard_true, action_unlock0);

    let p0 = ExecUnit::new(0, vec![p_lock0]);
    let p1 = ExecUnit::new(1, vec![p_lock1]);
    let p2 = ExecUnit::new(2, vec![p_unlock1]);
    let p3 = ExecUnit::new(3, vec![p_unlock0]);

    let p = vec![p0, p1, p2, p3];
    let ret = Process::new("P", p);

    ret
}

fn q_def() -> Process<SharedVars> {
    /* Create Process "P" */
    let p_lock1 = ProcessTrans::new("lock1", 1, guard_lock1, action_lock1);
    let p_lock0 = ProcessTrans::new("lock0", 2, guard_lock0, action_lock0);
    let p_unlock0 = ProcessTrans::new("unlock0", 3, guard_true, action_unlock0);
    let p_unlock1 = ProcessTrans::new("unlock1", 0, guard_true, action_unlock1);

    let p0 = ExecUnit::new(0, vec![p_lock1]);
    let p1 = ExecUnit::new(1, vec![p_lock0]);
    let p2 = ExecUnit::new(2, vec![p_unlock0]);
    let p3 = ExecUnit::new(3, vec![p_unlock1]);

    let p = vec![p0, p1, p2, p3];
    let ret = Process::new("Q", p);

    ret
}

fn processes_def() -> Vec<Process<SharedVars>> {
    let p = p_def();
    let q = q_def();
    let process = vec![p, q];

    process
}

pub fn main() {
    /* visualize each process */
    let process = processes_def();
    process[0].visualize("res/m_lock.dot");

    let r: SharedVars = Default::default();
    let s = State::new(r);
    let lts = concurrent_composition(process, s);
    lts.visualize("res/m_lock.dot");
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
