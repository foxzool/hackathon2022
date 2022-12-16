use crate::core::MOVE;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::move_store::MoveStore;

pub struct G1TwistStore<'a> {
    cube: &'a mut RubiksCubeIndexModel,
    moves: Vec<MOVE>,
}

impl<'a> G1TwistStore<'a> {
    pub fn new(cube: &'a mut RubiksCubeIndexModel) -> Self {
        let moves = vec![
            MOVE::L,
            MOVE::LPRIME,
            MOVE::L2,
            MOVE::R,
            MOVE::RPRIME,
            MOVE::R2,
            MOVE::U,
            MOVE::UPRIME,
            MOVE::U2,
            MOVE::D,
            MOVE::DPRIME,
            MOVE::D2,
            MOVE::F2,
            MOVE::B2,
        ];

        Self { cube, moves }
    }
}

impl MoveStore for G1TwistStore<'_> {
    fn get_moves(&self) -> &[MOVE] {
        self.moves.as_slice()
    }

    fn get_move_string(&self, ind: usize) -> String {
        todo!()
    }

    fn r#move(&self, ind: usize) {
        todo!()
    }

    fn invert(&self, ind: usize) {
        todo!()
    }
}
