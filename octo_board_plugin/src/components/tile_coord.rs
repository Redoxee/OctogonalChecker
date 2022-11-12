use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::*;

use crate::components::shape::*;
use crate::game_plugin::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct TileCoord
{
    pub x: i32,
    pub y: i32,
}

impl Add for TileCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for TileCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for TileCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[allow(dead_code)]
pub fn get_index_from_coord(coord: TileCoord) -> Option<usize> {
    let width = TILES_ON_ROW as i32;
    let height = TILES_ON_COL as i32;
    if coord.x < 0 || coord.y < 0 || coord.x >= width || coord.y >= height {
        return Option::None
    }

    if coord.y < height - 1 || coord.x % 2 == 0{
        return Some(get_index_from_coord_unsafe(coord))
    }
    else {
        return Option::None
    }
}

#[allow(dead_code)]
pub fn get_index_from_coord_unsafe(coord: TileCoord) -> usize {
    let width = TILES_ON_ROW as i32;
    let height = TILES_ON_COL as i32;
    
    if coord.y < height - 1 {
        return (coord.y * width + coord.x) as usize
    }
    else
    {
        return (coord.y * width + (coord.x) / 2) as usize
    }
}

#[allow(dead_code)]
pub fn get_coord_from_index(index : usize) -> TileCoord {
    let index = index as i32;
    let width = TILES_ON_ROW as i32;
    let height = TILES_ON_COL as i32;
    let mut result = TileCoord{x: index % width, y: index / width}; 
    if result.y == height - 1 {
        result.x = result.x * 2;
    }

    return result;
}

#[allow(dead_code)]
pub fn get_tile_shape(coord: TileCoord) -> Shape {
    if coord.x % 2 == 0 {
        return Shape::Quad;
    }
    else {
        return Shape::Octo;
    }
}

#[allow(dead_code)]
pub fn get_tile_shape_from_index(tile_index: usize) -> Shape {
    if tile_index % TILES_ON_ROW % 2 == 0 {
        return Shape::Quad;
    }
    else {
        return Shape::Octo;
    }
}