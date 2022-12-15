use crate::core::{CORNER, EDGE};
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::RubiksCube;

pub struct GoalG3G4;

impl Goal for GoalG3G4 {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        let i_cube = cube
            .as_any()
            .downcast_ref::<RubiksCubeIndexModel>()
            .expect("G1G2::is_satisfied() called with non-index model");

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

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}
