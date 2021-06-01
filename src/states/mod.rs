use bevy::prelude::Plugin;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Pause,
    Running
}

pub struct States;
impl Plugin for States {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_state(AppState::Pause);
    }
}