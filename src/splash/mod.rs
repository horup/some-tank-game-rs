use bevy::{prelude::*};

use crate::{AppState, Config, DelayState, Hud};

pub struct SplashPlugin;

fn show_splash(mut hud:ResMut<Hud>) {
    hud.center_text = "Some Tank Game!".into();
    hud.top_right_text = format!("Build Date\n{}", env!("BUILD_DATE"));
    hud.top_left_text = format!("Version\nV{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));
    hud.bottom_center_text = "Use W,A,S,D to drive your tank.\nUse the mouse to point and shoot!".into();
}

fn hide_splash(mut hud:ResMut<Hud>) {
    hud.clear_texts();
}

fn update(config:Res<Config>, mouse_input:Res<Input<MouseButton>>, mut app_state:ResMut<DelayState<AppState>>, mut hud:ResMut<Hud>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let time = if config.quick() == false { 0.5 } else { 0.0 };
        app_state.set(AppState::InGame, time);

        if config.quick() == false {
            hud.fade(time, time, Color::BLACK);
        }
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Splash).with_system(show_splash.system()));
        app.add_system_set(SystemSet::on_exit(AppState::Splash).with_system(hide_splash.system()));
        app.add_system_set(SystemSet::on_update(AppState::Splash).with_system(update.system()));
    }
}