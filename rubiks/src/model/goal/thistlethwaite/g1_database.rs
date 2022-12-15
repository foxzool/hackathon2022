use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::pattern_database::G1PatternDatabase;
use crate::model::RubiksCube;

pub struct G1Database {
    database: G1PatternDatabase,
}

impl G1Database {
    pub fn new(database: G1PatternDatabase) -> Self {
        Self { database }
    }
}

impl Goal for G1Database {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        todo!()
    }

    fn get_description(&self) -> String {
        "包含从每个棱方向排列到 G1 的移动次数".to_string()
    }

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}

impl DatabaseGoal for G1Database {
    type Database = G1PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> Self::Database {
        todo!()
    }
}
