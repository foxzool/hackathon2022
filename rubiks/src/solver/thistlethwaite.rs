use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::searcher::PatternDatabaseIndexer;

pub struct ThistlethwaiteCubeSolver {}

impl ThistlethwaiteCubeSolver {
    pub fn initialize() {}

    fn index_g1_database() {
        let i_cube = RubiksCubeIndexModel::default();
        let indexer = PatternDatabaseIndexer::default();
    }
}
