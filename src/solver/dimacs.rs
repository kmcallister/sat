use {Instance, Assignment, Literal};
use solver::Solver;

use std::io::{self, Write, BufRead};
use std::process::Command;
use std::str::FromStr;

use tempfile;

pub struct Dimacs<CmdFactory> {
    cmd_factory: CmdFactory,
}

impl<CmdFactory> Dimacs<CmdFactory>
    where CmdFactory: Fn() -> Command,
{
    pub fn new(cmd_factory: CmdFactory) -> Dimacs<CmdFactory>
        where CmdFactory: Fn() -> Command,
    {
        Dimacs {
            cmd_factory: cmd_factory,
        }
    }

    pub fn write_instance<W>(&self, writer: &mut W, instance: &Instance)
        where W: Write,
    {
        write!(writer, "p cnf {} {}\n",
            instance.num_vars, instance.cnf_clauses.len()).unwrap();
        for c in &instance.cnf_clauses {
            for l in c {
                let n = (l.var + 1) as isize;
                write!(writer, "{} ", if l.negated { -n } else { n }).unwrap();
            }
            write!(writer, "0\n").unwrap();
        }
    }

    pub fn read_solution<R>(&self, reader: &mut R, num_vars: usize) -> Option<Assignment>
        where R: BufRead,
    {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "UNSAT\n" {
            return None;
        }

        assert!(line == "SAT\n", "expected \"SAT\"");

        let mut assignment: Vec<_> = (0..num_vars).map(|i| Literal {
            var: i,
            negated: false,
        }).collect();

        loop {
            line.clear();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }

            for tok in line.split_whitespace() {
                let i = isize::from_str(tok).unwrap();
                if i == 0 {
                    continue;
                }

                if i < 0 {
                    assignment[(-i - 1) as usize].negated = true;
                }
            }
        }

        Some(Assignment {
            assignment: assignment,
        })
    }
}

impl<CmdFactory> Solver for Dimacs<CmdFactory>
    where CmdFactory: Fn() -> Command,
{
    fn solve(&self, instance: &Instance) -> Option<Assignment> {
        let mut in_file = tempfile::NamedTempFile::new().unwrap();
        let out_file = tempfile::NamedTempFile::new().unwrap();

        self.write_instance(&mut in_file, instance);

        let mut cmd = (self.cmd_factory)();

        // don't check the return code because minisat returns
        // non-zero on success :(
        let _ = cmd.arg(in_file.path())
           .arg(out_file.path())
           .spawn().unwrap().wait();

        let out_file = out_file.reopen().unwrap();
        let mut out_file = io::BufReader::new(out_file);

        self.read_solution(&mut out_file, instance.num_vars)
    }
}
