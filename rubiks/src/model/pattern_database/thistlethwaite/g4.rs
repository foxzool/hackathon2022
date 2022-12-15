use crate::model::pattern_database::PatternDatabase;
use crate::model::RubiksCube;
use crate::utils::NibbleArray;

pub struct G4PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
}

impl G4PatternDatabase {}

impl PatternDatabase for G4PatternDatabase {
    fn init(size: usize) -> Self {
        todo!()
    }

    fn get_database_index(&self, cube: &impl RubiksCube) -> u32 {
        todo!()
    }

    fn set_num_moves(&mut self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }

    fn set_num_moves_by_index(&mut self, ind: i32, num_moves: u8) -> bool {
        todo!()
    }

    fn get_num_moves(&self, cube: &impl RubiksCube) -> u8 {
        todo!()
    }

    fn get_num_moves_by_index(&self, ind: u32) -> u8 {
        todo!()
    }

    fn get_num_moves_ex(&self, cube: &impl RubiksCube, bound_hint: u8, depth: u8) -> u8 {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }

    fn get_num_items(&self) -> usize {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
    }

    fn to_file(&self, file_path: &str) {
        todo!()
    }

    fn from_file(&mut self, file_path: &str) {
        todo!()
    }

    fn inflate(&self) -> Vec<u8> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}
