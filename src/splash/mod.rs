use bevy::{prelude::*};

use crate::{AppState, DelayState, Hud};

pub struct SplashPlugin;

fn show_splash(mut hud:ResMut<Hud>) {
    hud.center_text = "Some Tank Game!\nBy Horup".into();
}

fn hide_splash(mut hud:ResMut<Hud>) {
    hud.clear();
}

fn update(mouse_input:Res<Input<MouseButton>>, mut app_state:ResMut<DelayState<AppState>>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        app_state.set(AppState::InBetweenGames, 1.0);
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Splash).with_system(show_splash.system()));
        app.add_system_set(SystemSet::on_exit(AppState::Splash).with_system(hide_splash.system()));
        app.add_system_set(SystemSet::on_update(AppState::Splash).with_system(update.system()));
    }
}