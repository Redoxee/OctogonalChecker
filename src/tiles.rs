use ggez::{*, graphics::MeshBuilder};
use glam::*;

use crate::shape_style::*;
use crate::utils::*;

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
    fn build_mesh(&self, style: ShapeStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            ShapeStyle::Base => graphics::Color::new(0.6, 0.6, 0.6, 1_f32),
            ShapeStyle::Highlight => graphics::Color::new(0.3, 0.4, 0.5, 1_f32),
            ShapeStyle::Hovered => graphics::Color::new(0.8, 0.8, 0.8, 1_f32),
            ShapeStyle::Press => graphics::Color::new(0.9, 0.9, 0.9, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();

        match style {
            ShapeStyle::Highlight => {
                mesh_builder.polygon(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(2.)), &self.verts.to_vec(), graphics::Color::YELLOW).unwrap();
            },
            _=> {},
        }
    }

    fn contain_position(&self, position: &Vec2) -> bool{
        return position_in_poly(&self.verts, position)
    }

    fn position(&self) -> Vec2 {
        self.position
    }
}

impl Shape for QuadTile{
    fn build_mesh(&self, style: ShapeStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            ShapeStyle::Base => graphics::Color::new(0.7, 0., 0., 1_f32),
            ShapeStyle::Highlight => graphics::Color::new(0.1, 0., 0.3, 1_f32),
            ShapeStyle::Hovered => graphics::Color::new(0.8, 0.3, 0.3, 1_f32),
            ShapeStyle::Press => graphics::Color::new(0.9, 0.5, 0.5, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();
        match style {
            ShapeStyle::Highlight => {
                mesh_builder.polygon(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(2.)), &self.verts.to_vec(), graphics::Color::YELLOW).unwrap();
            },
            _=> {},
        }
    }

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