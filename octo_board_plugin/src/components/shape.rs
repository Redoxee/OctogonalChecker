use bevy::prelude::*;

use std::fmt::{
    *,
    Display
};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub enum Shape {
    Octo,
    Quad,
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        
        match self {
            Shape::Quad=> write!(f, "Quad"),
            Shape::Octo=> write!(f, "Octo"),
        }
    }
}