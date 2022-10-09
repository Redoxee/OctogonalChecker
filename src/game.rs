use glam::*;

pub const MAX_PAWN_NUMBER: usize = 4;
pub const AI_PAUSE_TIME: f64 = 0.5_f64;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum PlayerSide {
    Bottom,
    Top,
}

pub struct DrawingContext {
    pub time: f64,
}

impl PlayerSide {
    pub fn reverse(self) -> PlayerSide {
        match self {
            PlayerSide::Bottom => PlayerSide::Top,
            PlayerSide::Top => PlayerSide::Bottom,
        }
    }
}