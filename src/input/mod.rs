use bevy::prelude::*;

use crate::{AppState, Waypoint};

mod input;
use input::*;

mod waypoints_marker;
use waypoints_marker::*;

pub struct InputPlugin;

#[derive(Debug)]
pub enum WaypointEvent {
    Added(Waypoint),
    Removed(Waypoint),
    Clear
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<WaypointEvent>();
        app.insert_resource(WaypointMarkerSprites::default());
        app.add_system_set_to_stage(CoreStage::Update, 
            SystemSet::on_update(AppState::InGame)
            .with_system(input_system.system().label("input"))
            .with_system(waypoints_marker_system.system().after("input"))
        );
    }
}