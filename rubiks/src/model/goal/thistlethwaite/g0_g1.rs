use crate::core::EDGE;
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::RubiksCube;

pub struct GoalG0G1;

impl Goal for GoalG0G1 {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        let disoriented = 1u8;
        let num_edges = 12u8;
        let i_cube = cube
            .as_any()
            .downcast_ref::<RubiksCubeIndexModel>()
            .expect("G0G1::is_satisfied() called with non-index model");

        for i in 0..num_edges {
            if i_cube.get_edge_orientation(EDGE::try_from(i).unwrap()) == disoriented {
                return false;
            }
        }

        true
    }

    fn get_description(&self) -> String {
        "调整所有边的方向，使它们无需转动四分之一 U 或 D 即可移回原位。".to_string()
    }

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}
