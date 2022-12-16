pub use g1_twist_store::*;
pub use g2_twist_store::*;
pub use g3_twist_store::*;

use crate::core::MOVE;

mod g1_twist_store;
mod g2_twist_store;
mod g3_twist_store;

pub trait MoveStore {
    fn get_moves(&self) -> &[MOVE];

    fn get_move(&self, ind: usize) -> MOVE {
        self.get_moves()[ind]
    }
    fn get_move_string(&self, ind: usize) -> String;

    fn get_num_moves(&self) -> usize {
        self.get_moves().len()
    }

    fn is_valid_move(&self, r#move: MOVE) -> bool {
        for m in self.get_moves() {
            if r#move == *m {
                return true;
            }
        }

        false
    }

    fn r#move(&self, ind: usize);

    fn invert(&self, ind: usize);
}
