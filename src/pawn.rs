use glam::Vec2;

use crate::game::*;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Pawn {
    pub player : PlayerSide,
    pub table_index: usize,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct PawnArray {
    pub tile_indexes: [usize; MAX_PAWN_NUMBER],
    pub count: usize,
}

impl PawnArray {
    pub fn new() -> PawnArray {
        PawnArray {
            tile_indexes: [0; MAX_PAWN_NUMBER],
            count: 0,
        }
    }
}