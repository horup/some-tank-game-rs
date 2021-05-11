use bevy::prelude::*;
use rand::random;
use crate::{Tile, Tilemap};

pub fn test_system(mut query:Query<(&mut Tilemap,)>, time:Res<Time>) {
    query.for_each_mut(|(mut tilemap, )| {
        
       // let tiles = tilemap.tiles_mut();
       // let r = random::<usize>() % tiles.len();
    });
}