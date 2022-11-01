use::std::ops;

use bevy::{prelude::*};
use crate::tiles::*;

pub const GRID_SIDE: usize = 4;
pub const NUMBER_OF_TILES: usize = (GRID_SIDE * 2 + 1)  * GRID_SIDE + GRID_SIDE + 1;
pub const TILES_ON_SIDE: usize = GRID_SIDE * 2 + 1;
pub const TILES_ON_ROW: usize = TILES_ON_SIDE;
pub const TILES_ON_COL: usize = GRID_SIDE + 1;


#[derive(Clone, Copy)]
pub struct BoundingBox{
    x: f32,
    right: f32,
    top: f32,
    y: f32,
}

impl BoundingBox{
    fn new(x: f32, y: f32, width: f32, height:f32) -> BoundingBox{
        BoundingBox{
            x,
            y,
            right: x + width,
            top: y + height,
        }
    }

    fn is_in(&self, position: &Vec2) -> bool{
        position.x >= self.x && position.y >= self.y && position.x <= self.right && position.y <= self.top
    }
}


#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub struct TileCoord{
    pub x:i32,
    pub y:i32,
}

impl ops::Add<TileCoord> for TileCoord{
    type Output = TileCoord;

    fn add(self, rhs: TileCoord) -> TileCoord {
        TileCoord{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::fmt::Display for TileCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{0},{1}]", self.x, self.y)
    }
}


#[derive(Clone, Copy)]
pub struct Grid {
    pub tiles: [GridTile; NUMBER_OF_TILES],
    pub position: Vec2,
    pub scale: f32,
    pub width: f32,
    pub bounding_box: BoundingBox,
}

impl Grid{
    pub fn new(octogon_ratio: f32, position: Vec2, scale: f32, thickness: f32) -> Grid{
        let tile_on_side = GRID_SIDE * 2 + 1;
        let bb_scale = scale * (tile_on_side + 1) as f32;
        let mut grid = Grid{
            tiles: [GridTile::None; NUMBER_OF_TILES],
            position,
            scale,
            bounding_box: BoundingBox::new(position.x - scale, position.y - scale, bb_scale, bb_scale),
            width: GRID_SIDE as f32 * scale * 2.,
        };

        let half_tile_gap = scale;
        let tile_gap = half_tile_gap * 2.;
        let octo_delta = Vec2::new(half_tile_gap, half_tile_gap);

        let mut array_index = 0;
        for y_index in 0..=GRID_SIDE {
            for x_index in 0..=GRID_SIDE {
                let position = Vec2::new(x_index as f32, y_index as f32) * tile_gap;
                grid.tiles[array_index] = GridTile::Quad(QuadTile::new(position, octogon_ratio, scale, thickness));
                array_index += 1;

                if x_index < GRID_SIDE && y_index < GRID_SIDE{
                    grid.tiles[array_index] = GridTile::Octo(OctoTile::new(position + octo_delta, octogon_ratio, scale, thickness));
                    array_index += 1;
                }
            }
        }

        grid
    }

    pub fn get_index_from_coord(coord: TileCoord) -> Option<usize> {
        let width = TILES_ON_ROW as i32;
        let height = TILES_ON_COL as i32;
        if coord.x < 0 || coord.y < 0 || coord.x >= width || coord.y >= height {
            return Option::None
        }

        if coord.y < height - 1 || coord.x % 2 == 0{
            return Some(Grid::get_index_from_coord_unsafe(coord))
        }
        else {
            return Option::None
        }
    }

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
    pub fn get_tile_shape(coord: TileCoord) -> TileShape {
        if coord.x % 2 == 0 {
            return TileShape::Quad;
        }
        else {
            return TileShape::Octo;
        }
    }

    pub fn get_tile_shape_from_index(tile_index: usize) -> TileShape {
        if tile_index % TILES_ON_ROW % 2 == 0 {
            return TileShape::Quad;
        }
        else {
            return TileShape::Octo;
        }
    }

    pub fn get_tile_at(&self, position: Vec2) -> isize{
        if !self.bounding_box.is_in(&position) {
            return -1
        }

        let coord = position - self.position;
        let base_x = (coord.x / self.scale / 2_f32).floor() as i32;
        let base_y = (coord.y / self.scale / 2_f32).floor() as i32;

        let mut possible_coord = Vec::new();
        possible_coord.push(TileCoord{x: base_x * 2, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2 + 1, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2 + 2, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2, y: base_y + 1});
        possible_coord.push(TileCoord{x: base_x * 2 + 2, y: base_y + 1});

        let position = position - self.position;
        for coord in possible_coord {
            match Grid::get_index_from_coord(coord) {
                Some(index)=> {
                    if self.tiles[index].contain_position(&position) {
                        return index as isize
                    }
                },

                None=> {},
            }
        }
        
        return -1
    }
}

impl ops::Index<TileCoord> for Grid {
    type Output = GridTile;
    
    fn index(&self, index: TileCoord) -> &GridTile {
        let index = Grid::get_index_from_coord_unsafe(index);
        return &self.tiles[index]
    }
}
