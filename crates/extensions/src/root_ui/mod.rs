use bevy::{ecs::schedule::ShouldRun, prelude::*};


pub struct RootNode;

// query does not work in run criteria systems due to a bug, use resource
struct RootNodeSpawned(bool);

fn spawn_root_node(mut commands:Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>, mut spawned:ResMut<RootNodeSpawned>) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type:PositionType::Absolute,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    }).insert(RootNode);

    spawned.0 = true;
}

fn has_root_ui(res:Res<RootNodeSpawned>) -> ShouldRun {
    if res.0 {
        return ShouldRun::No;
    } else {
        return ShouldRun::Yes;
    }
}

pub struct RootUIPlugin;

impl Plugin for RootUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RootNodeSpawned(false));
        app.add_system_set_to_stage(CoreStage::First, SystemSet::new().with_system(spawn_root_node.system()).with_run_criteria(has_root_ui.system()));
    }
}
