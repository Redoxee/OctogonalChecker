use glam::*;

use crate::game::*;
use crate::grid::*;
use crate::brain::*;
use crate::board::*;
use crate::pawn::*;

pub struct InGameState {
    grid: Grid,
    player_option: PlayerOption,
    prev_mouse_position: Vec2,
    was_pressed: bool,
    is_pressed: bool,
    hovered_tile: isize,

    pub board_state : BoardState,
    selected_pawn : isize,
    possible_plays: Vec<usize>,
    top_player_pawn : Pawn,
    bottom_player_pawn : Pawn,
    previous_states: Vec<BoardState>,
    ai_timer: f64,
}

#[derive(PartialEq, Eq)]
pub enum PlayerOption {
    OnePlayer,
    TwoPlayer,
}

pub enum InGameResult {
    Winner(PlayerSide),
    None,
}

impl InGameState {
    pub fn new(player_option: PlayerOption) -> InGameState{
        
        let grid_position = Vec2::new(120., 120.);
        let grid = Grid::new(0.3, grid_position, 60., 5.);
        let mut game = InGameState{
            grid,
            player_option,
            board_state: BoardState {
                tiles: [Option::None; NUMBER_OF_TILES],
                current_player: PlayerSide::Bottom,
                top_pawns: PawnArray::new(),
                bottom_pawns: PawnArray::new(),
            },
            was_pressed: false,
            is_pressed: false,
            hovered_tile: -1,
            prev_mouse_position: Vec2::new(-1_f32, -1_f32),
            selected_pawn: -1,
            possible_plays: Vec::new(),
            top_player_pawn: Pawn {player: PlayerSide::Top, table_index: 0},
            bottom_player_pawn: Pawn{player: PlayerSide::Bottom, table_index: 0},
            previous_states: Vec::new(),
            ai_timer: -1_f64,
        };

        game.board_state.add_pawn(TileCoord{x:3, y: 0}, PlayerSide::Top);
        game.board_state.add_pawn(TileCoord{x:4, y: 0}, PlayerSide::Top);
        game.board_state.add_pawn(TileCoord{x:5, y: 0}, PlayerSide::Top);
        game.board_state.add_pawn(TileCoord{x:4, y: 1}, PlayerSide::Top);

        game.board_state.add_pawn(TileCoord{x:3, y: 3}, PlayerSide::Bottom);
        game.board_state.add_pawn(TileCoord{x:4, y: 4}, PlayerSide::Bottom);
        game.board_state.add_pawn(TileCoord{x:5, y: 3}, PlayerSide::Bottom);

        return game;
    }

    pub fn unselect_pawn(&mut self) {
        self.selected_pawn = -1;
        self.possible_plays.clear();
    }
}

impl InGameState {
    pub fn update(&mut self) -> InGameResult {
        let mouse_position = Vec2::new();

        if mouse_position != self.prev_mouse_position
        {
            self.hovered_tile = self.grid.get_tile_at(mouse_position);
        }

        self.prev_mouse_position = mouse_position;
        self.was_pressed = self.is_pressed;
        self.is_pressed =  false;
        
        let ai_play = self.board_state.current_player == PlayerSide::Top && self.player_option == PlayerOption::OnePlayer;
        if ai_play {
            if self.ai_timer > 0_f64 {
                let delta = 0.16_f32;
                self.ai_timer = self.ai_timer - delta;
                if self.ai_timer <= 0_f64 {
                    match Brain::search_best_play(&self.board_state, 10) {
                        Some(best_play) => {
                            self.previous_states.push(self.board_state.clone());
                            self.board_state  = self.board_state.make_move(best_play.0, best_play.1);
                        },

                        None => {}
                    }
                }
            }
        }
        else {
            if !self.was_pressed && self.is_pressed {
            if self.hovered_tile > -1 {
                if self.selected_pawn < 0 {
                    match &mut self.board_state.tiles[self.hovered_tile as usize] {
                        Some(pawn) => {
                            if self.board_state.current_player == pawn.player {
                                self.selected_pawn = self.hovered_tile;
                                let player_side = pawn.player;
                                self.possible_plays = self.board_state.get_possible_plays(self.hovered_tile as usize, player_side);
                            }
                        }

                        None => {},
                    }
                }
                else 
                {
                    if self.hovered_tile != self.selected_pawn && self.possible_plays.contains(&(self.hovered_tile as usize)) {
                        let source_index = self.selected_pawn as usize;
                        self.unselect_pawn();

                        self.previous_states.push(self.board_state.clone());
                        self.board_state  = self.board_state.make_move(source_index, self.hovered_tile as usize);

                        self.ai_timer = AI_PAUSE_TIME;
                    }
                    else {
                        self.unselect_pawn();
                        match &mut self.board_state.tiles[self.hovered_tile as usize] {
                            Some(pawn) => {
                                if self.board_state.current_player == pawn.player {
                                    self.selected_pawn = self.hovered_tile;
                                    let player_side = pawn.player;
                                    self.possible_plays = self.board_state.get_possible_plays(self.hovered_tile as usize, player_side);
                                }
                            }
    
                            None => {},
                        }
                    }
                }
            }
            else {
                if self.selected_pawn > -1 {
                    self.unselect_pawn();
                }
            }
        }

        if self.board_state.top_pawns.count == 0 {
            return InGameResult::Winner(PlayerSide::Bottom);
        }
        else if self.board_state.bottom_pawns.count == 0 {
            return InGameResult::Winner(PlayerSide::Top);
        }
        
        InGameResult::None
    }
}
}
