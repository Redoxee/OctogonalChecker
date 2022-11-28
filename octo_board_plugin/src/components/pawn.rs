use bevy::prelude::Component;

use crate::components::tile_coord::*;

pub enum PlayerSide {
    Top,
    Bottom
}

#[derive(Component)]
pub struct Pawn {
    pub player_side : PlayerSide,
    pub position : Option<TileCoord>
}