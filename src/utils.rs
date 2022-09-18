use ggez::{graphics::{MeshBuilder}};
use glam::*;

use crate::shape_style::*;

pub trait Shape {
    fn build_mesh(&self, style: ShapeStyle, mesh_builder: &mut MeshBuilder);
    fn contain_position(&self, position: &Vec2) -> bool;
    fn position(&self) -> Vec2;
}