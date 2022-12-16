use crate::core::{CORNER, EDGE};
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;

pub struct GoalG3G4;

impl Goal for GoalG3G4 {
    fn is_satisfied(&self, i_cube: &RubiksCubeIndexModel) -> bool {
        let num_edges = 12u8;
        let num_corners = 8u8;

        for i in 0..(num_edges - 2) {
            if i_cube.get_edge_index(EDGE::try_from(i).unwrap()) != i {
                return false;
            }
        }

        for i in 0..(num_corners - 3) {
            if i_cube.get_corner_index(CORNER::try_from(i).unwrap()) != i {
                return false;
            }
        }

        true
    }

    fn get_description(&self) -> String {
        "只用一半的旋转来解决立方体".to_string()
    }

    fn index(&self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }
}
