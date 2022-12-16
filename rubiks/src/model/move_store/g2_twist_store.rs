use crate::core::MOVE;
use crate::model::move_store::MoveStore;

pub struct G2TwistStore {
    moves: Vec<MOVE>,
}

impl G2TwistStore {
    pub fn new() -> Self {
        let moves = vec![
            MOVE::L2,
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

        Self { moves }
    }
}

impl MoveStore for G2TwistStore {
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
