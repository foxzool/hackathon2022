use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::PatternDatabase;

pub trait DatabaseGoal {
    type Database: PatternDatabase;
    fn get_database(&self) -> Self::Database;
    fn get_mut_database(&mut self) -> Self::Database;
    fn get_num_moves(&self, cube: &RubiksCubeIndexModel) -> u8 {
        self.get_database().get_num_moves(cube)
    }
    fn get_num_moves_by_index(&self, ind: u32) -> u8 {
        self.get_database().get_num_moves_by_index(ind)
    }
    fn get_database_index(&self, cube: &RubiksCubeIndexModel) -> u32 {
        self.get_database().get_database_index(cube)
    }
    fn is_satisfied(&self, _cube: &RubiksCubeIndexModel) -> bool {
        self.get_database().is_full()
    }

    fn index(&mut self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        self.get_mut_database().set_num_moves(cube, num_moves)
    }
    fn index_by_id(&mut self, ind: u8, num_moves: u8) -> bool {
        self.get_mut_database()
            .set_num_moves_by_index(ind, num_moves)
    }
}
