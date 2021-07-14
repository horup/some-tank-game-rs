use bevy::prelude::*;

use crate::AssetCache;

pub fn preload(mut asset_cache:ResMut<AssetCache>, asset_server:Res<AssetServer>) {
    let mut t = |s| {
        asset_cache.track_untyped(&asset_server.load_untyped(s));
    };

    t("levels.json");
   
    t("fonts/default.ttf");

    t("music/Zander Noriega - Fight Them Until We Cant.ogg");

    t("imgs/explosion.png");
    t("imgs/objects.png");
    t("imgs/tanks.png");
    t("imgs/tiles.png");
    t("imgs/white.png");

    t("maps/1.tmx");
    t("maps/2.tmx");
    t("maps/3.tmx");
    t("maps/4.tmx");

    t("sfx/boom_1.ogg");
    t("sfx/boom_2.ogg");
    t("sfx/boom_3.ogg");
    t("sfx/go.ogg");
    t("sfx/great.ogg");
    t("sfx/shoot_1.ogg");
    t("sfx/shoot_2.ogg");
    t("sfx/too_bad.ogg");
    t("sfx/won.ogg");




}