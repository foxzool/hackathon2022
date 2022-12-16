use crate::core::{CORNER, EDGE};
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::{CombinationIndexer, PatternDatabase};
use crate::utils::NibbleArray;

pub struct G2PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
    combo_indexer: CombinationIndexer,
}

impl PatternDatabase for G2PatternDatabase {
    fn init(size: usize) -> Self {
        Self {
            database: NibbleArray::new(1082565),
            size,
            num_items: 0,
            combo_indexer: CombinationIndexer::new(12, 4),
        }
    }

    fn get_database_index(&self, i_cube: &RubiksCubeIndexModel) -> u32 {
        let num_edges = 12;

        let mut edge_combo = [0; 4];
        let mut combo_ind = 0;
        let mut i = 0u8;
        while i < num_edges && combo_ind < 4 {
            let edge_ind = i_cube.get_edge_index(EDGE::try_from(i).unwrap());
            if edge_ind == EDGE::FR as u8
                || edge_ind == EDGE::FL as u8
                || edge_ind == EDGE::BL as u8
                || edge_ind == EDGE::BR as u8
            {
                combo_ind += 1;
                edge_combo[combo_ind] = i;
            }
            i += 1;
        }

        let rank = self.combo_indexer.rank(&edge_combo);
        let corner_orientations = [
            i_cube.get_corner_orientation(CORNER::ULB),
            i_cube.get_corner_orientation(CORNER::URB),
            i_cube.get_corner_orientation(CORNER::URF),
            i_cube.get_corner_orientation(CORNER::ULF),
            i_cube.get_corner_orientation(CORNER::DLF),
            i_cube.get_corner_orientation(CORNER::DLB),
            i_cube.get_corner_orientation(CORNER::DRB),
        ];

        let orientation_num = corner_orientations[0] as u32 * 729
            + corner_orientations[1] as u32 * 243
            + corner_orientations[2] as u32 * 81
            + corner_orientations[3] as u32 * 27
            + corner_orientations[4] as u32 * 9
            + corner_orientations[5] as u32 * 3
            + corner_orientations[6] as u32;

        rank * 2187 + orientation_num
    }

    fn set_num_moves(&mut self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }

    fn set_num_moves_by_index(&mut self, ind: i32, num_moves: u8) -> bool {
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
