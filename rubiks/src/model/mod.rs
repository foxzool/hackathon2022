use std::any::Any;

pub mod goal;
pub mod index_model;
pub mod pattern_database;
pub mod searcher;
pub mod solver;
pub mod std_model;

pub trait RubiksCube {
    fn as_any(&self) -> &dyn Any;
}
