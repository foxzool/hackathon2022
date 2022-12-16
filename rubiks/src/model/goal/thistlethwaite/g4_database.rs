use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::G4PatternDatabase;

pub struct G4Database {
    database: G4PatternDatabase,
}

impl G4Database {
    pub fn new(database: G4PatternDatabase) -> Self {
        Self { database }
    }
}

impl Goal for G4Database {
    fn is_satisfied(&self, cube: &RubiksCubeIndexModel) -> bool {
        todo!()
    }

    fn get_description(&self) -> String {
        "包含将四分体中的所有角配对并将所有边放入其切片所需的移动次数。".to_string()
    }

    fn index(&self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }
}

impl DatabaseGoal for G4Database {
    type Database = G4PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> Self::Database {
        todo!()
    }
}
