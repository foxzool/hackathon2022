use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::G1PatternDatabase;

pub struct G1DatabaseGoal<'a> {
    database: &'a G1PatternDatabase,
}

impl G1DatabaseGoal<'_> {
    pub fn new(database: &G1PatternDatabase) -> G1DatabaseGoal {
        G1DatabaseGoal { database }
    }
}

impl Goal for G1DatabaseGoal<'_> {
    fn is_satisfied(&self, cube: &RubiksCubeIndexModel) -> bool {
        todo!()
    }

    fn get_description(&self) -> String {
        "包含从每个棱方向排列到 G1 的移动次数".to_string()
    }

    fn index(&self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool {
        todo!()
    }
}

impl DatabaseGoal for G1DatabaseGoal<'_> {
    type Database = G1PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> Self::Database {
        todo!()
    }
}
