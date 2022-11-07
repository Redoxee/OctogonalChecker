use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

/// Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
    pub map: Vec<Tile>,
    pub octogon_on_side: usize,
}

impl TileMap {
    pub fn create(octogon_on_side: usize) -> Self {
        let mut map = Vec::new();
        for y in 0..(octogon_on_side + 1) {
            for x in 0..(octogon_on_side + 1) {
                map.push(Tile::Quad(x * 2, y * 2));
                if x < octogon_on_side && y < octogon_on_side {
                    map.push(Tile::Octo(x * 2 + 1, y * 2 + 1));
                }
            }
        }

        Self {
            map,
            octogon_on_side
        }
    }

}

impl Deref for TileMap {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}