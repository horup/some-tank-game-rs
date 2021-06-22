use std::ops::Deref;

use bevy::{asset::{AssetLoader, LoadedAsset}, prelude::*, reflect::{TypeUuid, Uuid}};

pub struct JsonLoader;

pub struct Json(pub serde_json::Value);

impl Deref for Json {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TypeUuid for Json {
    const TYPE_UUID: Uuid = Uuid::from_bytes([0x88 as u8, 0xe5, 0xa7, 0xb1, 0xc8, 0xed, 0x48, 0x03, 0xa6, 0x42, 0x61, 0xff, 0x83, 0xaf, 0xaa, 0x0c]);
}

impl AssetLoader for JsonLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            println!("test");
            let value = serde_json::from_slice::<serde_json::Value>(bytes);
            match value {
                Ok(value) => {
                    println!("test");
                    load_context.set_default_asset(LoadedAsset::new(Json(value)));
                    return Ok(());
                },
                Err(_) => {
                    
                },
            }
            Err(anyhow::anyhow!("unable to load json"))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
pub struct JsonLoaderPlugin;

impl Plugin for JsonLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset_loader(JsonLoader);
        app.add_asset::<Json>();
    }
}