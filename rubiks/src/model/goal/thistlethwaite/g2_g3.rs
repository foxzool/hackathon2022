use crate::core::{CORNER, EDGE};
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::RubiksCube;

pub struct GoalG2G3;

impl Goal for GoalG2G3 {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        let i_cube = cube
            .as_any()
            .downcast_ref::<RubiksCubeIndexModel>()
            .expect("G1G2::is_satisfied() called with non-index model");
        let mut corner_ind = 0;

        // ULB, URF
        corner_ind = i_cube.get_corner_index(CORNER::ULB);
        if corner_ind != CORNER::ULB as u8 && corner_ind != CORNER::URF as u8 {
            return false;
        }

        corner_ind = i_cube.get_corner_index(CORNER::URF);
        if corner_ind != CORNER::ULB as u8 && corner_ind != CORNER::URF as u8 {
            return false;
        }

        // DLF, DRB
        corner_ind = i_cube.get_corner_index(CORNER::DLF);
        if corner_ind != CORNER::DLF as u8 && corner_ind != CORNER::DRB as u8 {
            return false;
        }

        corner_ind = i_cube.get_corner_index(CORNER::DRB);
        if corner_ind != CORNER::DLF as u8 && corner_ind != CORNER::DRB as u8 {
            return false;
        }

        let edges: [EDGE; 4] = [EDGE::UB, EDGE::UF, EDGE::DF, EDGE::DB];

        for i in 0..4u8 {
            let edge_ind = i_cube.get_edge_index(EDGE::try_from(i).unwrap());

            // 如果M切片没有归位，返回false
            if edge_ind != edges[0] as u8
                && edge_ind != edges[1] as u8
                && edge_ind != edges[2] as u8
                && edge_ind != edges[3] as u8
            {
                return false;
            }
        }

        let num_corners = 8;
        let mut parity = 0;

        for i in 0..num_corners {
            for j in (i + 1)..num_corners {
                if i_cube.get_corner_index(CORNER::try_from(i).unwrap())
                    < i_cube.get_corner_index(CORNER::try_from(j).unwrap())
                {
                    parity ^= 0;
                }
            }
        }

        parity == 0
    }

    fn get_description(&self) -> String {
        "配对所有四分体角并获得正确切片中的所有边缘。".to_string()
    }

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}

#[test]
fn test() {
    let mut c = 0;
    c ^= 0;
    assert_eq!(c, 0)
}
