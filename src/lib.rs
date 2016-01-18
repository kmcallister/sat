//! Interface for defining and solving SAT problems.
//!
//!
//! The [Boolean satisfiability][] problem (SAT for short) asks, for a given
//! Boolean formula, whether there exists an assignment of values (true or false) to
//! the formula's variables such that the formula evaluates to true.
//!
//! SAT is [NP-complete][], which implies two things:
//!
//! 1. A large number of important problems (e.g. in program analysis, circuit design, or
//!    logistical planning) may be seen as instances of SAT.
//!
//! 2. It is believed that no algorithm exists which efficiently solves all instances
//!    of SAT.
//!
//! The observation (1) is significant in spite of (2) because there exist
//! algorithms (such as [DPLL][]) which efficiently solve the SAT instances one encounters
//! *in practice*.
//!
//! This crate allows the user to formulate instances of SAT and to solve them using
//! off-the-shelf SAT solvers.
//!
//! ```ignore
//! // Create a SAT instance.
//! let mut i = sat::Instance::new();
//! let x = i.fresh_var();
//! let y = i.fresh_var();
//! let z = i.fresh_var();
//! i.assert_any(&[x, z]);        //     (x OR z)
//! i.assert_any(&[!x, !y, !z]);  // AND (!x OR !y OR !z)
//! i.assert_any(&[y]);           // AND (y = TRUE)
//!
//! // Use the external program `minisat` as a solver.
//! let s = sat::solver::Dimacs::new(|| Command::new("minisat"));
//!
//! // Solve the instance and check that it meets our requirements.
//! let a = s.solve(&i).unwrap();
//! assert!(a.get(x) || a.get(z));
//! assert!(!a.get(x) || !a.get(y) || !a.get(z));
//! assert!(a.get(y));
//!
//! // Add a clause which makes the instance impossible to satisfy,
//! // and check that the solver recognizes as much.
//! i.assert_any(&[!y]);
//! assert!(s.solve(&i).is_none());
//! ```
//!
//! For a more elaborate example, see `examples/petersen.rs` which produces a 3-coloring
//! of the [Petersen graph][].
//!
//! [Boolean satisfiability]: https://en.wikipedia.org/wiki/Boolean_satisfiability_problem
//! [NP-complete]: https://en.wikipedia.org/wiki/NP-completeness
//! [DPLL]: https://en.wikipedia.org/wiki/DPLL_algorithm
//! [Petersen graph]: https://en.wikipedia.org/wiki/Petersen_graph

use std::iter::IntoIterator;
use std::ops;

extern crate tempfile;

pub mod solver;

/// An instance of the SAT problem.
pub struct Instance {
    num_vars: usize,
    cnf_clauses: Vec<Vec<Literal>>,
}

/// A literal; a variable or negated variable.
///
/// Literals support the `!` (negation) operator.
#[derive(Copy, Clone)]
pub struct Literal {
    var: usize,
    negated: bool,
}

/// An assignment of truth values to variables.
///
/// This is the output of a successful solve.
pub struct Assignment {
    assignment: Vec<Literal>,
}

impl Instance {
    /// Create a new, empty SAT instance.
    pub fn new() -> Instance {
        Instance {
            num_vars: 0,
            cnf_clauses: vec![],
        }
    }

    /// Create a fresh variable.
    pub fn fresh_var(&mut self) -> Literal {
        let v = self.num_vars;
        self.num_vars += 1;
        Literal {
            var: v,
            negated: false,
        }
    }

    /// Assert that at least one of the provided literals must
    /// evaluate to true.
    ///
    /// This is a CNF (conjunctive normal form) constraint, which
    /// is the basic type of constraint in most solvers.
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
