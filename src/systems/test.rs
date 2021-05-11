use bevy::prelude::*;
use rand::random;
use crate::{Tile, Tilemap};

pub fn test_system(mut query:Query<(&mut Tilemap,)>, time:Res<Time>) {
}