pub mod menu_state;
pub mod in_game_state;
pub mod game_over_state;

use crate::pawn::*;

use menu_state::*;
use in_game_state::*;
use game_over_state::*;

use crate::{game};

pub enum GameState {
    MenuState(MenuState),
    InGame(InGameState),
    GameOver(GameOverState),
}

pub enum GameStateResult {
    None,
    NextState(GameState),
}

impl GameState {
    pub fn update(&mut self, ctx: &mut ggez::Context) -> Result<GameStateResult, ggez::GameError> {
        match self {
            GameState::MenuState(state) => match state.update(ctx) {
                Ok(option) => {
                    match option {
                        MenuOption::None => {
                            Ok(GameStateResult::None)
                        },
                        MenuOption::OnePlayer => {
                            Ok(GameStateResult::NextState(GameState::InGame(InGameState::new(PlayerOption::OnePlayer))))
                        },
                        MenuOption::TwoPlayer => {
                            Ok(GameStateResult::NextState(GameState::InGame(InGameState::new(PlayerOption::TwoPlayer))))
                        }
                    }
                }
                Err(error)=> {
                    return Result::Err(error);
                }
            },
            GameState::InGame(state) => match state.update(ctx) {
                InGameResult::None => {
                    Ok(GameStateResult::None)
                }
                InGameResult::Winner(winner) => {
                    Ok( GameStateResult::NextState(GameState::GameOver(GameOverState{
                        winner_pawn: Pawn{
                            player: winner,
                            table_index: 0,
                        }
                    })))
                }
            },
            GameState::GameOver(state) => 
            { 
                state.update(ctx)?;
                Ok(GameStateResult::None)
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, drawing_context: &mut game::DrawingContext) -> Result<(), ggez::GameError> {
        match self {
            GameState::MenuState(state) => state.draw(ctx, drawing_context),
            GameState::InGame(state) => state.draw(ctx, drawing_context),
            GameState::GameOver(state) => state.draw(ctx, drawing_context),
        }
    }
}