use std::io;

extern crate sat;

fn main() {
    let mut i = sat::Instance::new();
    let x = i.fresh_var();
    let y = i.fresh_var();
    i.assert_any(&[x, y]);
    i.assert_any(&[!x, !y]);

    let s = sat::solver::dimacs::Dimacs::new(|| panic!());
    s.write_instance(&mut io::stdout(), &i);
}
