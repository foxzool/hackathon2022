use crate::core::{CORNER, EDGE};
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::{
    CombinationIndexer, PatternDatabase, UnorderedPairSetIndexer,
};
use crate::model::RubiksCube;
use crate::utils::NibbleArray;

pub struct G3PatternDatabase {
    database: NibbleArray,
    size: usize,
    num_items: usize,
    combo_indexer: CombinationIndexer,
    pair_set_indexer: UnorderedPairSetIndexer,
}

impl G3PatternDatabase {
    fn get_tetrad_pair(
        &self,
        i_cube: &RubiksCubeIndexModel,
        tetrad_pair: &mut [u8; 2],
        c1: CORNER,
        c2: CORNER,
    ) {
        let num_corners = 8;
        let mut combo_ind = 0;

        let mut i = 0u8;
        while i < num_corners && combo_ind < 2 {
            let corner_ind = i_cube.get_corner_index(c1);
            if corner_ind == c1 as u8 || corner_ind == c2 as u8 {
                combo_ind += 1;
                tetrad_pair[combo_ind] = i;
            }
            i += 1;
        }
    }
}

impl PatternDatabase for G3PatternDatabase {
    fn init(size: usize) -> Self {
        Self {
            database: NibbleArray::new(352800),
            size,
            num_items: 0,
            combo_indexer: CombinationIndexer::new(8, 4),
            pair_set_indexer: UnorderedPairSetIndexer::init(8),
        }
    }

    fn get_database_index(&self, cube: &impl RubiksCube) -> u32 {
        let i_cube = cube
            .as_any()
            .downcast_ref::<RubiksCubeIndexModel>()
            .expect("Wasn't a trusty cube");
        let num_edges = 12u8;
        let num_corners = 8u8;

        let mut tetrad_pairs = [[0; 2]; 4];
        self.get_tetrad_pair(i_cube, &mut tetrad_pairs[0], CORNER::ULB, CORNER::URF);
        self.get_tetrad_pair(i_cube, &mut tetrad_pairs[1], CORNER::DLF, CORNER::DRB);
        self.get_tetrad_pair(i_cube, &mut tetrad_pairs[2], CORNER::URB, CORNER::ULF);
        self.get_tetrad_pair(i_cube, &mut tetrad_pairs[3], CORNER::DLB, CORNER::DRF);

        let corner_rank = self.pair_set_indexer.rank(&tetrad_pairs);

        let mut edge_map = [0u8; 12];

        edge_map[EDGE::UB as usize] = 0;
        edge_map[EDGE::UR as usize] = 1;
        edge_map[EDGE::UF as usize] = 2;
        edge_map[EDGE::UL as usize] = 3;
        edge_map[EDGE::DF as usize] = 4;
        edge_map[EDGE::DL as usize] = 5;
        edge_map[EDGE::DB as usize] = 6;
        edge_map[EDGE::DR as usize] = 7;

        let mut edge_combo = [0u8; 4];
        let mut edge_combo_ind = 0;
        let mut i = 0u8;
        while i < num_edges && edge_combo_ind < 4 {
            let edge_ind = i_cube.get_edge_index(EDGE::try_from(i).unwrap());
            if edge_ind == EDGE::UB as u8
                || edge_ind == EDGE::UR as u8
                || edge_ind == EDGE::UF as u8
                || edge_ind == EDGE::UL as u8
            {
                edge_combo_ind += 1;
                edge_combo[edge_combo_ind] = edge_map[i as usize];
            }
            i += 1;
        }

        let edge_rank = self.combo_indexer.rank(&edge_combo);
        let mut parity = 0u8;

        for i in 0..num_corners {
            for j in (i + 1)..num_corners {
                if i_cube.get_corner_index(CORNER::try_from(i).unwrap())
                    < i_cube.get_corner_index(CORNER::try_from(j).unwrap())
                {
                    parity ^= 1;
                }
            }
        }

        // 2520 = 8C2*6C2*4C2. 8C2 is the number of ways to choose 2 corners from 8 corners.
        (edge_rank * 2520 + corner_rank) * 2 + parity as u32
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
