use std::process::{self, Command};

use sat::solver::Solver;

extern crate sat;

#[test]
fn smoke_test() {
    let mut i = sat::Instance::new();
    let x = i.fresh_var();
    let y = i.fresh_var();
    let z = i.fresh_var();
    i.assert_any(&[x, z]);
    i.assert_any(&[!x, !y, !z]);
    i.assert_any(&[y]);

    let s = sat::solver::Dimacs::new(|| {
        let mut c = Command::new("minisat");
        c.stdout(process::Stdio::null());
        c
    });

    let a = s.solve(&i).unwrap();
    assert!(a.get(x) || a.get(z));
    assert!(!a.get(x) || !a.get(y) || !a.get(z));
    assert!(a.get(y));

    i.assert_any(&[!y]);
    assert!(s.solve(&i).is_none());
}
