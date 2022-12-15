use crate::model::RubiksCube;
pub use database_goal::*;

mod database_goal;

trait Goal {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool;
    fn get_description(&self) -> String;
    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool;
}