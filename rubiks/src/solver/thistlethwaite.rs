use crate::model::goal::thistlethwaite::G1DatabaseGoal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::pattern_database::{
    G1PatternDatabase, G2PatternDatabase, G3PatternDatabase, G4PatternDatabase,
};
use crate::model::searcher::PatternDatabaseIndexer;

pub struct ThistlethwaiteCubeSolver {
    g1_db: G1PatternDatabase,
    g2_db: G2PatternDatabase,
    g3_db: G3PatternDatabase,
    g4_db: G4PatternDatabase,
}

impl ThistlethwaiteCubeSolver {
    fn index_g1_database(&self) {
        let i_cube = RubiksCubeIndexModel::default();
        let indexer = PatternDatabaseIndexer::default();
        let goal = G1DatabaseGoal::new(self.g1_db.clone());
    }
}
