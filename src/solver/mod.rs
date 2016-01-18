use {Instance, Assignment};

pub use self::dimacs::Dimacs;

pub mod dimacs;

pub trait Solver {
    fn solve(&self, instance: &Instance) -> Option<Assignment>;
}
