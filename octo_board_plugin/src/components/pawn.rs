use bevy::prelude::Component;

use crate::components::tile_coord::*;

#[derive(Component)]
pub enum Pawn {
    Top(Option<TileCoord>),
    Bottom(Option<TileCoord>)
}