use crate::core::{CORNER, EDGE};
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;

pub struct GoalG1G2;

impl Goal for GoalG1G2 {
    fn is_satisfied(&self, i_cube: &RubiksCubeIndexModel) -> bool {
        let disoriented = 1u8;
        let num_corners = 12u8;

        // 如果有任意一个角块没有对齐，返回false
        for i in 0..num_corners {
            if i_cube.get_corner_orientation(CORNER::try_from(i).unwrap()) == disoriented {
                return false;
            }
        }

        let edges: [EDGE; 4] = [EDGE::FR, EDGE::FL, EDGE::BL, EDGE::BR];

        for i in 0..4usize {
            let edge_ind = i_cube.get_edge_index(edges[i]);

            // 如果棱没有归位，返回false
            if edge_ind != edges[0] as u8
                && edge_ind != edges[1] as u8
                && edge_ind != edges[2] as u8
                && edge_ind != edges[3] as u8
            {
                return false;
            }
        }

        true
    }

    fn get_description(&self) -> String {
        "定位所有角，并将 FR、FL、BL、BR 放置在正确的切片 (E) 中。".to_string()
    }

    fn index(&self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }
}
