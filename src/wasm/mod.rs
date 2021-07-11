use bevy::prelude::*;
use bevy::window::WindowResized;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width:f32, height:f32);
}

fn resizer(mut windows:ResMut<Windows>, mut window_resized_events: EventWriter<WindowResized>, wd:Res<WindowDescriptor>) {
    let window = web_sys::window().expect("no global `window` exists");
    let mut width:f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let mut height:f32 = window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
        if window.width() != width || window.height() != height {
            width = if width < wd.resize_constraints.min_width { wd.resize_constraints.min_width } else { width };
            width = if width > wd.resize_constraints.max_width { wd.resize_constraints.max_width } else { width };
            height = if height < wd.resize_constraints.min_height { wd.resize_constraints.min_height } else { height };
            height = if height > wd.resize_constraints.max_height { wd.resize_constraints.max_height } else { height };

            window.update_actual_size_from_backend(width as u32, height as u32);
            window_resized_events.send(WindowResized {
                id:window.id(),
                height:height,
                width:width
            });
            resize_canvas(width, height);
            info!("Resizing to {:?},{:?}", width, height);
        }
    }
}


pub struct WASMPlugin;

impl Plugin for WASMPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
        app.add_system(resizer.system());
    }
}