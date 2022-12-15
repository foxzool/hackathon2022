use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::pattern_database::G3PatternDatabase;
use crate::model::RubiksCube;

pub struct G3Database {
    database: G3PatternDatabase,
}

impl G3Database {
    pub fn new(database: G3PatternDatabase) -> Self {
        Self { database }
    }
}

impl Goal for G3Database {
    fn is_satisfied(&self, cube: &impl RubiksCube) -> bool {
        todo!()
    }

    fn get_description(&self) -> String {
        "包含将四分体中的所有角配对并将所有边放入其切片所需的移动次数。".to_string()
    }

    fn index(&self, cube: &impl RubiksCube, num_moves: u8) -> bool {
        todo!()
    }
}

impl DatabaseGoal for G3Database {
    type Database = G3PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> Self::Database {
        todo!()
    }
}
