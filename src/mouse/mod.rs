use bevy::{prelude::*, render::camera::Camera};
use crate::{AppState, GameCamera};

#[derive(Default, Clone, Copy)]
pub struct Mouse {
    pub pos_screen:Vec2,
    pub pos_world:Vec3,
    pub has_touch:bool
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, SystemLabel)]
pub struct MouseSystem;

pub fn mouse_input_system(mut mouse:ResMut<Mouse>, camera:Query<(&Camera, &Transform, &GameCamera)>, mut mouse_moved_events:EventReader<CursorMoved>, windows:Res<Windows>) {
    for e in mouse_moved_events.iter() {
        mouse.pos_screen = e.position;
    }

    update_position_world(&camera, &windows, &mut mouse);
}

fn update_position_world(camera: &Query<(&Camera, &Transform, &GameCamera)>, windows: &Res<Windows>, mouse: &mut ResMut<Mouse>) {
    if let Ok((camera, transform, _)) = camera.single() {
        let wnd = windows.get_primary().expect("could not get primary monitor");
        let wnd_size = Vec2::new(wnd.width(), wnd.height());
       
        let ndc_pos = mouse.pos_screen / wnd_size * 2.0 - Vec2::new(1.0, 1.0);

        let projection_matrix = camera.projection_matrix;
        let transform_matrix = transform.compute_matrix();

        let pos_world = (transform_matrix * (projection_matrix.inverse() * ndc_pos.extend(0.0).extend(1.0))).truncate();
        mouse.pos_world = pos_world;
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Mouse::default());
        app.add_system_set_to_stage(CoreStage::Update, 
            SystemSet::on_update(AppState::InGame)
            .with_system(mouse_input_system.system().label(MouseSystem))
        );
    }
}

