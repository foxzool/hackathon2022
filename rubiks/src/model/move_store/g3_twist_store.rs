use crate::core::MOVE;
use crate::model::move_store::MoveStore;

pub struct G3TwistStore {
    moves: Vec<MOVE>,
}

impl G3TwistStore {
    pub fn new() -> Self {
        let moves = vec![MOVE::L2, MOVE::R2, MOVE::U2, MOVE::D2, MOVE::F2, MOVE::B2];

        Self { moves }
    }
}

impl MoveStore for G3TwistStore {
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
