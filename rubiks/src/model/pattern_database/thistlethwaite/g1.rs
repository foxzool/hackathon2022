use crate::core::EDGE;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::PatternDatabase;
use crate::model::RubiksCube;
use crate::utils::NibbleArray;

pub struct G1PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
}

impl PatternDatabase for G1PatternDatabase {
    fn init(size: usize) -> Self {
        Self {
            database: NibbleArray::new(size),
            size,
            num_items: 0,
        }
    }

    fn get_database_index(&self, cube: &impl RubiksCube) -> u32 {
        let i_cube: &RubiksCubeIndexModel = cube
            .as_any()
            .downcast_ref::<RubiksCubeIndexModel>()
            .expect("Wasn't a trusty cube");

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
