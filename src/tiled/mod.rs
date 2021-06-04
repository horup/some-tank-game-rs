use std::path::Path;

use bevy::asset::{BoxedFuture, LoadContext, LoadedAsset};
use bevy::{asset::AssetLoader, prelude::*};
use bevy::reflect::{TypeUuid, Uuid};

pub struct TiledPlugin;
struct TiledMapLoader;
impl AssetLoader for TiledMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path:String = "assets/".to_string() + load_context.path().to_str().expect("not valid path");
            let map = tiled::parse_with_path(bytes, Path::new(&path));
            if let Ok(map) = map {
                load_context.set_default_asset(LoadedAsset::new(TiledMap {
                    map
                }));

                return Ok(());
            }
            Err(anyhow::anyhow!("unable to load map"))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}
pub struct TiledMap {
    pub map:tiled::Map
}

// 88e307b1-c8ed-4102-a646-63ff86afaa0c
impl TypeUuid for TiledMap {
    const TYPE_UUID: Uuid = Uuid::from_bytes([0x88 as u8, 0xe3, 0x07, 0xb1, 0xc8, 0xed, 0x41, 0x02, 0xa6, 0x46, 0x63, 0xff, 0x86, 0xaf, 0xaa, 0x0c]);
}


fn load_assets(maps:Res<AssetServer>) {
    let h:Handle<TiledMap> = maps.load("maps/1.tmx");
    println!("{:?}", h);
}


impl Plugin for TiledPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<TiledMap>();
        app.add_asset_loader(TiledMapLoader);
        app.add_startup_system(load_assets.system());
    }
}