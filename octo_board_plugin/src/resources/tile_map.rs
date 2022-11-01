use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

/// Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
    width: u16,
    height: u16,
    map: Vec<Tile>,
}

impl TileMap {
    pub fn create(width: u16, height: u16) -> Self {
        let mut map = Vec::new();
        for _y in 0..(height - 1) {
            for _x in 0..(width - 1) {
                map.push(Tile::Quad);
                map.push(Tile::Octo);
            }

            map.push(Tile::Quad);
        }

        for _x in 0..width {
            map.push(Tile::Quad);
        }

        Self {
            width,
            height,
            map,
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