//! Interface to SAT solvers.

use {Instance, Assignment};

pub use self::dimacs::Dimacs;

pub mod dimacs;

/// Trait for SAT solvers.
pub trait Solver {
    /// Solve an instance and return the satisfying assignment, or
    /// `None` if no such assignment exists.
    fn solve(&self, instance: &Instance) -> Option<Assignment>;
}
