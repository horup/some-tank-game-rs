use std::io::Cursor;

use bevy::{prelude::*, winit::WinitWindows};
use extensions::{Icon, IconDir};

pub struct NativePlugin;

fn set_icon(world:&mut World) {
    let world = world.cell();
    let winit_windows = world.get_resource::<WinitWindows>().unwrap();
    let mut windows = world.get_resource_mut::<Windows>().unwrap();

    for bevy_window in windows.iter_mut() {
        let id = bevy_window.id();
        let window = winit_windows.get_window(id).unwrap();
        let icon = include_bytes!("../../assets/icon.ico") as &[u8];
        let icon = Cursor::new(icon);
        let icon = IconDir::read(icon).unwrap();
        let icon = icon.entries().first().unwrap();
        let img = icon.decode().unwrap();
        let icon = Icon::from_rgba(img.rgba_data().into(), icon.width() , icon.height()).unwrap();
        window.set_window_icon(Some(icon));
    }
}

impl Plugin for NativePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(set_icon.exclusive_system());
    }
}