use std::iter::IntoIterator;
use std::ops;

extern crate tempfile;

pub mod solver;

pub struct Instance {
    num_vars: usize,
    cnf_clauses: Vec<Vec<Literal>>,
}

#[derive(Copy, Clone)]
pub struct Literal {
    var: usize,
    negated: bool,
}

pub struct Assignment {
    assignment: Vec<Literal>,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            num_vars: 0,
            cnf_clauses: vec![],
        }
    }

    pub fn fresh_var(&mut self) -> Literal {
        let v = self.num_vars;
        self.num_vars += 1;
        Literal {
            var: v,
            negated: false,
        }
    }

    pub fn assert_any<'a, I>(&mut self, lits: I)
        where I: IntoIterator<Item = &'a Literal>
    {
        let lits = lits.into_iter();
        self.cnf_clauses.push(lits.cloned().collect());
    }
}

impl ops::Not for Literal {
    type Output = Literal;

    fn not(self) -> Literal {
        Literal {
            negated: !self.negated,
            ..self
        }
    }
}

impl Assignment {
    /// Get the value assigned to a literal.
    pub fn get(&self, lit: Literal) -> bool {
        lit.negated ^ (!self.assignment[lit.var].negated)
    }
}
