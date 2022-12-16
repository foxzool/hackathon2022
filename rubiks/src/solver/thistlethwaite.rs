use crate::model::goal::thistlethwaite::{
    G1DatabaseGoal, G2DatabaseGoal, G3DatabaseGoal, G4DatabaseGoal,
};
use crate::model::goal::Goal;
use crate::model::index_model::RubiksCubeIndexModel;
use crate::model::move_store::{G1TwistStore, G2TwistStore, G3TwistStore, TwistStore};
use crate::model::pattern_database::{
    G1PatternDatabase, G2PatternDatabase, G3PatternDatabase, G4PatternDatabase, PatternDatabase,
};
use crate::model::searcher::PatternDatabaseIndexer;
use crate::solver::CubeSolver;

pub struct ThistlethwaiteCubeSolver {
    g1_db: G1PatternDatabase,
    g2_db: G2PatternDatabase,
    g3_db: G3PatternDatabase,
    g4_db: G4PatternDatabase,
    solving: bool,
}

impl ThistlethwaiteCubeSolver {
    fn index_g1_database(&self) {
        let mut i_cube = RubiksCubeIndexModel::default();
        let mut goal = G1DatabaseGoal::new(&self.g1_db);

        let mut twist_store = TwistStore::new();
        println!("Goal 1: {}", goal.get_description());
        PatternDatabaseIndexer::find_goal(&mut goal, &i_cube, &twist_store);
    }

    fn index_g2_database(&self) {
        let mut i_cube = RubiksCubeIndexModel::default();
        let mut goal = G2DatabaseGoal::new(&self.g2_db);

        let mut g1_twist_store = G1TwistStore::new();
        println!("Goal 2: {}", goal.get_description());
        PatternDatabaseIndexer::find_goal(&mut goal, &i_cube, &mut g1_twist_store);
    }

    fn index_g3_database(&self) {
        let mut i_cube = RubiksCubeIndexModel::default();
        let mut goal = G3DatabaseGoal::new(&self.g3_db);

        let mut g2_twist_store = G2TwistStore::new();
        println!("Goal 3: {}", goal.get_description());
        PatternDatabaseIndexer::find_goal(&mut goal, &i_cube, &mut g2_twist_store);
    }

    fn index_g4_database(&self) {
        let mut i_cube = RubiksCubeIndexModel::default();
        let mut goal = G4DatabaseGoal::new(&self.g4_db);

        let mut g3_twist_store = G3TwistStore::new();
        println!("Goal 4: {}", goal.get_description());
        PatternDatabaseIndexer::find_goal(&mut goal, &i_cube, &mut g3_twist_store);
    }

    pub fn init_database() {}
}

impl CubeSolver for ThistlethwaiteCubeSolver {
    fn solve_cube(&self) {
        todo!()
    }

    fn new() -> Self {
        Self {
            g1_db: G1PatternDatabase::new(2048),
            g2_db: G2PatternDatabase::new(1082565),
            g3_db: G3PatternDatabase::new(352800),
            g4_db: G4PatternDatabase::new(663552),
            solving: false,
        }
    }

    fn set_solving(&mut self, solving: bool) {
        todo!()
    }
}

#[test]
fn test_solver() {
    let solver = ThistlethwaiteCubeSolver::new();
    solver.index_g1_database();
    solver.index_g2_database();
    solver.index_g3_database();
    solver.index_g4_database();
}
