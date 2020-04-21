use rddsv::lts::*;
use rddsv::process::*;
use std::fmt;

#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SharedVars {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for SharedVars {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "x={} y={} z={}", self.x, self.y, self.z)
    }
}

/* User definition of guard and action
 * a = after, b = before, c = current
 */
fn action_x_1(_prop: Prop, a: &mut SharedVars, _b: &SharedVars) {
    a.x = 1;
}

fn action_y_0(_prop: Prop, a: &mut SharedVars, _b: &SharedVars) {
    a.y = 0;
}

fn action_y_1(_prop: Prop, a: &mut SharedVars, _b: &SharedVars) {
    a.y = 1;
}

fn action_z_1(_prop: Prop, a: &mut SharedVars, _b: &SharedVars) {
    a.z = 1;
}

fn p_def() -> Process<SharedVars> {
    /* Create Process "P" */
    let x1 = ProcessTrans::new("x=1", 1, guard_true, action_x_1);
    let y1 = ProcessTrans::new("y=1", 2, guard_true, action_y_1);
    let z1 = ProcessTrans::new("z=1", 3, guard_true, action_z_1);
    let y0 = ProcessTrans::new("y=0", 4, guard_true, action_y_0);

    let p0 = ExecUnit::new(0, vec![x1]);
    let p1 = ExecUnit::new(1, vec![y1]);
    let p2 = ExecUnit::new(2, vec![z1]);
    let p3 = ExecUnit::new(3, vec![y0]);
    let p4 = ExecUnit::new(4, vec![]);

    let p = vec![p0, p1, p2, p3, p4];
    let ret = Process::new("P", p);

    ret
}

fn processes_def() -> Vec<Process<SharedVars>> {
    let p = p_def();
    let process = vec![p];

    process
}

pub fn main() {
    /* visualize each process */
    let process = processes_def();
    process[0].visualize("res/m_test_1_P.dot");

    let r: SharedVars = Default::default();
    let s = State::new(r);
    let lts = concurrent_composition(process, s);
    lts.visualize("res/m_test_1.dot");
}

#[cfg(test)]
mod test {
    use super::*;
    use file_diff::diff_files;
    use std::fs::*;

    #[test]
    fn vis_process() {
        let process = processes_def();
        assert!(true);
    }

    #[test]
    fn vis_m_cas() {
        let process = processes_def();
        let r: SharedVars = Default::default();
        let s = State::new(r);
        let lts = concurrent_composition(process, s);
        assert!(true);
    }
}
