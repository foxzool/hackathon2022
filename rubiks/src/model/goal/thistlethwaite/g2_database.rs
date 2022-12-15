use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::pattern_database::G2PatternDatabase;
use crate::model::RubiksCube;

pub struct G2Database {
    database: G2PatternDatabase,
}

impl G2Database {
    pub fn new(database: G2PatternDatabase) -> Self {
        Self { database }
    }
}

impl Goal for G2Database {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        todo!()
    }

    fn get_description(&self) -> String {
        "包含用于定位所有角并定位 E 切片边缘的移动次数。".to_string()
    }

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}

impl DatabaseGoal for G2Database {
    type Database = G2PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> Self::Database {
        todo!()
    }
}
