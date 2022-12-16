pub use database_goal::*;

use crate::model::index_model::RubiksCubeIndexModel;

pub mod thistlethwaite;

mod database_goal;

pub trait Goal {
    fn is_satisfied(&self, cube: &RubiksCubeIndexModel) -> bool;
    fn get_description(&self) -> String;
    fn index(&self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool;
}
