use crate::model::goal::Goal;
use crate::model::pattern_database::PatternDatabase;
use crate::model::RubiksCube;

pub trait DatabaseGoal: Goal {
    type Database: PatternDatabase;
    fn get_database(&self) -> Self::Database;
    fn get_mut_database(&mut self) -> Self::Database;
    fn get_num_moves(&self, cube: &impl RubiksCube) -> u8 {
        self.get_database().get_num_moves(cube)
    }
    fn get_num_moves_by_index(&self, ind: u32) -> u8 {
        self.get_database().get_num_moves_by_index(ind)
    }
    fn get_database_index(&self, cube: &impl RubiksCube) -> u32 {
        self.get_database().get_database_index(cube)
    }
    fn is_satisfied(&self, _cube: &impl RubiksCube) -> bool {
        self.get_database().is_full()
    }

    fn index(&mut self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        self.get_mut_database().set_num_moves(cube, num_moves)
    }
}
