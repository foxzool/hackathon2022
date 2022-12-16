use crate::core::EDGE;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::PatternDatabase;
use crate::utils::NibbleArray;

#[derive(Debug, Clone)]
pub struct G1PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
}

impl PatternDatabase for G1PatternDatabase {
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

    fn get_database_index(&self, i_cube: &RubiksCubeIndexModel) -> u32 {
        i_cube.get_edge_orientation(EDGE::UB) as u32 * 1024
            + i_cube.get_edge_orientation(EDGE::UR) as u32 * 512
            + i_cube.get_edge_orientation(EDGE::UF) as u32 * 256
            + i_cube.get_edge_orientation(EDGE::UL) as u32 * 128
            + i_cube.get_edge_orientation(EDGE::FR) as u32 * 64
            + i_cube.get_edge_orientation(EDGE::FL) as u32 * 32
            + i_cube.get_edge_orientation(EDGE::BL) as u32 * 16
            + i_cube.get_edge_orientation(EDGE::BR) as u32 * 8
            + i_cube.get_edge_orientation(EDGE::DF) as u32 * 4
            + i_cube.get_edge_orientation(EDGE::DL) as u32 * 2
            + i_cube.get_edge_orientation(EDGE::DB) as u32 * 1
    }

    fn set_num_moves(&mut self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        self.set_num_moves_by_index(self.get_database_index(cube), num_moves)
    }

    fn set_num_moves_by_index(&mut self, ind: u32, num_moves: u8) -> bool {
        let old = self.get_num_moves_by_index(ind);

        if old == 0xF {
            self.num_items += 1;
        }

        if old > num_moves {
            self.database.set(ind as usize, num_moves);
            return true;
        }

        false
    }

    fn get_num_moves(&self, cube: &RubiksCubeIndexModel) -> u8 {
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
