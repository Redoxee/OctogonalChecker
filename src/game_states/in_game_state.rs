
use ggez::{
    *,
};

use glam::*;

use crate::game::*;
use crate::grid::*;
use crate::utils::*;
use crate::brain::*;
use crate::shape_style::*;
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

    #[allow(dead_code)]
    pub fn draw_tile_indexes(&self, ctx: &mut Context) {
        for index in 0..self.grid.tiles.len(){
            let label = graphics::Text::new(index.to_string());
            let hf = Vec2::new(label.width(&ctx)/ 2_f32, label.height(&ctx) / 2_f32);
            graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + self.grid.tiles[index].position() - hf)).unwrap();
        }
    }

    pub fn unselect_pawn(&mut self) {
        self.selected_pawn = -1;
        self.possible_plays.clear();
    }
}

impl InGameState {
    pub fn update(&mut self, ctx: &mut Context) -> InGameResult {
        let mouse_position = input::mouse::position(ctx);
        let mouse_position = Vec2::new(mouse_position.x, mouse_position.y);

        if mouse_position != self.prev_mouse_position
        {
            self.hovered_tile = self.grid.get_tile_at(mouse_position);
        }

        self.prev_mouse_position = mouse_position;
        self.was_pressed = self.is_pressed;
        self.is_pressed = input::mouse::button_pressed(ctx, event::MouseButton::Left);
        
        let ai_play = self.board_state.current_player == PlayerSide::Top && self.player_option == PlayerOption::OnePlayer;
        if ai_play {
            if self.ai_timer > 0_f64 {
                let delta = timer::duration_to_f64(timer::delta(ctx));
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

        /*
        let undo = input::keyboard::is_key_pressed(ctx, event::KeyCode::Z) && input::keyboard::is_mod_active(ctx, event::KeyMods::CTRL);
        if undo && !self.is_undo {
            match self.previous_states.pop() {
                Some(state) => {self.board_state = state;}
                None => {println!("History Empty");}
            }
        }

        self.is_undo = undo;
        // */
        }

        if self.board_state.top_pawns.count == 0 {
            return InGameResult::Winner(PlayerSide::Bottom);
        }
        else if self.board_state.bottom_pawns.count == 0 {
            return InGameResult::Winner(PlayerSide::Top);
        }
        
        InGameResult::None
    }

    pub fn draw(&mut self, ctx: &mut Context, drawing_context: &mut DrawingContext) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let mut mesh_builder = graphics::MeshBuilder::new();
        for index in 0..self.grid.tiles.len()
        {
            let tile = &self.grid.tiles[index];
            let mut style = ShapeStyle::Base;
            if self.hovered_tile == index as isize {
                if self.is_pressed {
                    style = ShapeStyle::Press
                }
                else {
                    style = ShapeStyle::Hovered
                }
            }
            else if self.possible_plays.contains(&index) {
                style = ShapeStyle::Highlight;
            }

            tile.build_mesh(style, &mut mesh_builder);
        }

        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.grid.position))?; 

        for index in 0..NUMBER_OF_TILES {
            if let Some(pawn) = self.board_state.tiles[index] {
                pawn.draw(drawing_context, ctx, self.grid.tiles[index].position() + self.grid.position, 2_f32, self.selected_pawn == index as isize);
            }
        }

        let label = graphics::Text::new("Current player : ");
        graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + Vec2::new(self.grid.width / 2. - label.width(ctx), -65.))).unwrap();
        let current_pawn = match self.board_state.current_player {PlayerSide::Bottom => self.bottom_player_pawn, PlayerSide::Top => self.top_player_pawn};
        current_pawn.draw(drawing_context, ctx, self.grid.position + Vec2::new(self.grid.width / 2. + 16_f32, -65.), 2_f32, false);

        let font_height = 24_f32;
        for index in 0..TILES_ON_ROW {
            let label = (('A' as u8 + index as u8) as char).to_string();
            let mut label = graphics::Text::new(label);
            label.set_font(graphics::Font::default(), graphics::PxScale{x: font_height, y: font_height});
            let position = self.grid.position + Vec2::new(self.grid.width / (GRID_SIDE as f32 * 2_f32 ) * index as f32 - label.width(ctx) / 2_f32, self.grid.width + self.grid.scale);
            graphics::draw(ctx, &label,graphics::DrawParam::default().dest(position))?;

            let label = (TILES_ON_ROW - index - 1).to_string();
            let mut label = graphics::Text::new(label);
            label.set_font(graphics::Font::default(), graphics::PxScale{x: font_height, y: font_height});
            let position = self.grid.position + Vec2::new(- self.grid.scale - label.width(ctx) / 2_f32, self.grid.width / (GRID_SIDE as f32 * 2_f32 ) * index as f32) - label.height(ctx) / 2_f32;
            graphics::draw(ctx, &label,graphics::DrawParam::default().dest(position))?;
        }

        let mut label = "".to_owned();
        if self.hovered_tile > -1 {
            let coord = Grid::get_coord_from_index(self.hovered_tile as usize);
            label.push_str(&format!("[{},{}] = {}", coord.x, coord.y, self.hovered_tile));
        }

        let label = graphics::Text::new(label);
        graphics::draw(ctx, &label, graphics::DrawParam::default())?;
        Ok(())
    }
}