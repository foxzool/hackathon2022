use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::PatternDatabase;
use crate::utils::NibbleArray;

pub struct G4PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
}

impl G4PatternDatabase {}

impl PatternDatabase for G4PatternDatabase {
    fn new(size: usize) -> Self {
        Self {
            database: NibbleArray::new(size),
            size,
            num_items: 0,
        }
    }

    fn get_database(&self) -> &NibbleArray {
        &self.database
    }

    fn get_database_index(&self, cube: &RubiksCubeIndexModel) -> u32 {
        todo!()
    }

    fn set_num_moves(&mut self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }

    fn set_num_moves_by_index(&mut self, ind: u32, num_moves: u8) -> bool {
        todo!()
    }

    fn get_num_moves(&self, cube: &RubiksCubeIndexModel) -> u8 {
        todo!()
    }

    fn get_num_moves_by_index(&self, ind: u32) -> u8 {
        todo!()
    }

    fn get_num_moves_ex(&self, cube: &RubiksCubeIndexModel, bound_hint: u8, depth: u8) -> u8 {
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
