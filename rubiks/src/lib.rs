use crate::camera::CameraPlugin;
use crate::viewer::ViewerPlugin;
use bevy::app::{App, Plugin};

pub mod camera;
pub mod core;
pub mod model;
pub mod parser;
pub mod utils;
pub mod viewer;

/// 魔方插件
pub struct BevyRubiksCubePlugin;

impl Plugin for BevyRubiksCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin).add_plugin(ViewerPlugin);
    }
}
