pub use korf::*;
pub use thistlethwaite::*;

mod korf;
mod thistlethwaite;

pub trait CubeSolver {
    fn solve_cube(&self);
    fn new() -> Self;
    fn set_solving(&mut self, solving: bool);
}
