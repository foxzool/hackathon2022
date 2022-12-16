use crate::model::goal::{DatabaseGoal, Goal};
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::G3PatternDatabase;

pub struct G3DatabaseGoal<'a> {
    database: &'a G3PatternDatabase,
}

impl G3DatabaseGoal<'_> {
    pub fn new(database: &G3PatternDatabase) -> G3DatabaseGoal {
        G3DatabaseGoal { database }
    }
}

impl Goal for G3DatabaseGoal<'_> {
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

impl DatabaseGoal for G3DatabaseGoal<'_> {
    type Database = G3PatternDatabase;

    fn get_database(&self) -> Self::Database {
        todo!()
    }

    fn get_mut_database(&mut self) -> &mut Self::Database {
        todo!()
    }
}
