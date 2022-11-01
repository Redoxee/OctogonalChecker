use bevy::{prelude::*};

use crate::shape_style::*;

pub trait Shape {
    fn contain_position(&self, position: &Vec2) -> bool;
    fn position(&self) -> Vec2;
}

#[derive(Clone, Copy)]
pub struct OctoTile {
    pub verts : [Vec2; 8],
    pub inner_verts : [Vec2; 8],
    pub position : Vec2,
}

#[derive(Clone, Copy)]
pub struct QuadTile {
    pub verts : [Vec2; 4],
    pub inner_verts : [Vec2; 4],
    pub position : Vec2,
}

pub enum TileShape {
    Quad,
    Octo,
}

impl OctoTile {
    pub fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> OctoTile {
        let half = octogon_ratio * size;

        let inner_size = size - thickness / 2.;
        let inner_half = octogon_ratio * (size - thickness/2.);
        let tile = OctoTile{
                verts:[
                    Vec2::new(size, half) + position,
                    Vec2::new(half, size) + position,
                    Vec2::new(-half, size) + position,
                    Vec2::new(-size, half) + position,
                    Vec2::new(-size, -half) + position,
                    Vec2::new(-half, -size) + position,
                    Vec2::new(half, -size) + position,
                    Vec2::new(size, -half) + position,
                ],

                inner_verts :[
                    Vec2::new(inner_size, inner_half) + position,
                    Vec2::new(inner_half, inner_size) + position,
                    Vec2::new(-inner_half, inner_size) + position,
                    Vec2::new(-inner_size, inner_half) + position,
                    Vec2::new(-inner_size, -inner_half) + position,
                    Vec2::new(-inner_half, -inner_size) + position,
                    Vec2::new(inner_half, -inner_size) + position,
                    Vec2::new(inner_size, -inner_half) + position,
                ],

                position,
        };

        tile
    }
}

impl QuadTile {
    pub fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> QuadTile {
        let size = size * (1. - octogon_ratio);
        let thickness = thickness / 2.;
        let tile= QuadTile{
            verts: [
                Vec2::new(0., -size) + position,
                Vec2::new(size, 0.) + position,
                Vec2::new(0., size) + position,
                Vec2::new(-size, 0.) + position,
            ],
  
            inner_verts:[
                Vec2::new(0., -size + thickness) + position,
                Vec2::new(size - thickness, 0.) + position,
                Vec2::new(0., size - thickness) + position,
                Vec2::new(-size + thickness, 0.) + position,
            ],

            position,
        };

        tile
    }
}

impl Shape for OctoTile{
    fn contain_position(&self, position: &Vec2) -> bool{
        return position_in_poly(&self.verts, position)
    }

    fn position(&self) -> Vec2 {
        self.position
    }
}

impl Shape for QuadTile{
    fn contain_position(&self, position: &Vec2) -> bool{
        return position_in_poly(&self.verts, position)
    }

    fn position(&self) -> Vec2 {
        self.position
    }
}

// from : https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
fn position_in_poly(vertices : &[Vec2], point : &Vec2) -> bool{
    let mut inside = false;
    let mut j = vertices.len() -1;
    for i in 0..vertices.len() {
        if  ((vertices[i].y > point.y) != (vertices[j].y > point.y)) &&
            (point.x < (vertices[j].x-vertices[i].x) * (point.y-vertices[i].y) / (vertices[j].y-vertices[i].y) + vertices[i].x) {
                inside = !inside;
            }

            j = i;
    }

    return inside;
}

#[derive(Clone, Copy)]
pub enum GridTile {
    Quad(QuadTile),
    Octo(OctoTile),
    None,
}

impl Shape for GridTile {
    fn contain_position(&self, position: &Vec2) -> bool {
        match self {
            GridTile::Quad(inner_tile) => inner_tile.contain_position(position),
            GridTile::Octo(inner_tile) => inner_tile.contain_position(position),
            GridTile::None => panic!()
        }
    }

    fn position(&self) -> Vec2 {
        match self {
            GridTile::Quad(inner_tile) => inner_tile.position(),
            GridTile::Octo(inner_tile) => inner_tile.position(),
            GridTile::None => panic!()
        }
    }
}