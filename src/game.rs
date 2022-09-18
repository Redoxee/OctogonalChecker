use ggez::{*,
    graphics,
    Context
};

use glam::*;

use crate::grid::*;
use crate::textures::*;
use crate::game_states::{
    *,
    menu_state::MenuState,
};

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
    pub game_textures: GameTextures,
    pub time: f64,
}

pub struct Game {
    game_state: GameState,
    grid: Grid,
    drawing_context: DrawingContext,
}

impl PlayerSide {
    pub fn reverse(self) -> PlayerSide {
        match self {
            PlayerSide::Bottom => PlayerSide::Top,
            PlayerSide::Top => PlayerSide::Bottom,
        }
    }
}

impl Game {
    pub fn new(ctx: &mut Context, grid_position: Vec2) -> GameResult<Game> {
        let grid = Grid::new(0.3, grid_position, 60., 5.);
        let game = Game {
            game_state: GameState::MenuState(MenuState::new()),
            grid,
            drawing_context: DrawingContext { 
                game_textures: GameTextures::new(ctx)?, 
                time: timer::duration_to_f64(timer::time_since_start(ctx)),
            }
        };
        
        return Ok(game);
    }
}

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        match self.game_state.update(ctx, &self.grid) {
            Ok(instruction) => match instruction {
                GameStateResult::None => {
                    return Result::Ok(());
                },
                GameStateResult::NextState(next_state) => {
                    self.game_state = next_state;
                    return Result::Ok(());
                },
            }
            Err(error) => {
                return Result::Err(error);
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.drawing_context.time = timer::duration_to_f64(timer::time_since_start(ctx));
        self.game_state.draw(ctx, &mut self.drawing_context)?;

        graphics::present(ctx)?;

        Ok(())
    }
}