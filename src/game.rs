use ggez::{*, graphics::MeshBuilder};
use glam::*;

use crate::shape_style::*;
use crate::tiles::*;
use crate::utils::*;
use crate::grid::*;
use crate::brain::*;

const MAX_PAWN_NUMBER: usize = 3;

#[derive(Clone, Copy)]
pub struct Pawn {
    pub player : PlayerSide,
    pub table_index: usize,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum PlayerSide {
    Bottom,
    Top,
}

#[derive(Clone, Copy)]
pub struct PawnArray {
    pub tile_indexes: [usize; MAX_PAWN_NUMBER],
    pub count: usize,
}

#[derive(Clone, Copy)]
pub struct BoardState {
    pub tiles: [Option<Pawn>; NUMBER_OF_TILES],
    pub current_player: PlayerSide,
    pub top_pawns: PawnArray,
    pub bottom_pawns: PawnArray,
}

pub struct Game {
    game_state : GameState,
}

pub enum GameState {
    InGame(InGameState),
    GameOver(GameOverState),
}

pub struct GameOverState {
    winner_pawn : Pawn,
}

pub struct InGameState {
    grid: Grid,
    prev_mouse_position: Vec2,
    was_pressed: bool,
    is_pressed: bool,
    is_undo : bool,
    hovered_tile: isize,
    board_state : BoardState,
    selected_pawn : isize,
    possible_plays: Vec<usize>,
    top_player_pawn : Pawn,
    bottom_player_pawn : Pawn,
    top_pawn_count : i8,
    bottom_pawn_count : i8,
    previous_states: Vec<BoardState>,
    ai_timer: f64,
}

impl Pawn {
    fn draw(&self, mesh_builder: &mut MeshBuilder, position: Vec2, scale: f32, is_selected: bool){
        let primary_color;
        let mut secondary_color;
        match self.player {
            PlayerSide::Bottom => {
                primary_color = graphics::Color::WHITE;
                secondary_color = graphics::Color::BLUE;
            }

            PlayerSide::Top => {
                primary_color = graphics::Color::BLUE;
                secondary_color = graphics::Color::WHITE;
            }
        }

        if is_selected {
            secondary_color = graphics::Color::YELLOW;
        }

        mesh_builder.circle(graphics::DrawMode::Fill(graphics::FillOptions::default()), position, scale, 0.1, primary_color).unwrap();
        mesh_builder.circle(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(3.)), position, scale, 0.1, secondary_color).unwrap();
    }
}

impl PlayerSide {
    pub fn reverse(self) -> PlayerSide {
        match self {
            PlayerSide::Bottom => PlayerSide::Top,
            PlayerSide::Top => PlayerSide::Bottom,
        }
    }
}

impl PawnArray {
    pub fn new() -> PawnArray {
        PawnArray {
            tile_indexes: [0; MAX_PAWN_NUMBER],
            count: 0,
        }
    }
}

impl BoardState {
    pub fn add_pawn(&mut self, coord: TileCoord, player: PlayerSide) {
        let tile_index = Grid::get_index_from_coord(coord).unwrap();
        match self.tiles[tile_index] {
            Some(_) => panic!(),
            None => (),
        }

        let pawn_array = match player {
            PlayerSide::Top => &mut self.top_pawns,
            PlayerSide::Bottom => &mut self.bottom_pawns,
        };

        self.tiles[tile_index] = Some(Pawn{player, table_index: pawn_array.count});
        (*pawn_array).tile_indexes[pawn_array.count] = tile_index;
        (*pawn_array).count = pawn_array.count + 1;
    }
    
    pub fn make_move(&self, source_index: usize, play_index: usize) -> BoardState {
        let mut board = self.clone();
        let pawn = board.tiles[source_index];

        match board.tiles[play_index] {
            None=>(),
            Some(pawn) => {
                let pawn_array = match pawn.player {
                    PlayerSide::Top => &mut board.top_pawns,
                    PlayerSide::Bottom => &mut board.bottom_pawns
                };
                
                if pawn_array.count > 1 {
                    let replacing_pawn_tile_index = pawn_array.tile_indexes[pawn_array.count - 1];
                    match board.tiles[replacing_pawn_tile_index] {
                        Some(mut other_pawn) => {
                            other_pawn.table_index = pawn.table_index;
                        },
                        None => { panic!()},
                    }
                    
                    (*pawn_array).tile_indexes[pawn.table_index] = pawn_array.tile_indexes[pawn_array.count - 1];
                }
                
                (*pawn_array).count = pawn_array.count - 1;
            }
        }

        board.tiles[play_index] = pawn;
        board.tiles[source_index] = Option::None;

        match pawn {
            Some(pawn) => {
                let tile_array = match pawn.player {
                    PlayerSide::Top => &mut board.top_pawns, 
                    PlayerSide::Bottom => &mut board.bottom_pawns
                };

                (*tile_array).tile_indexes[pawn.table_index] = play_index;
            }
            None => {panic!();}
        }

        return board;
    }
    
    pub fn get_possible_plays(&self, tile_index: usize, player_side: PlayerSide, grid: &Grid) -> Vec<usize>{
        let coord = Grid::get_coord_from_index(tile_index);
        let mut possible_plays = Vec::new();
        match grid[coord] {
            GridTile::Quad(_) => {
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y - 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y - 1}) {possible_plays.push(index)};
            },

            GridTile::Octo(_) => {
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y - 1}) {possible_plays.push(index)};

                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = Grid::get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y + 1}) {possible_plays.push(index)};
            },

            GridTile::None => panic!(),
        }

        possible_plays.retain(|&index| match self.tiles[index] {
            Some(pawn) => { pawn.player != player_side },
            None => true
        });

        return possible_plays;
    }
}

impl InGameState {
    fn new(grid : Grid) -> InGameState{
        let mut game = InGameState{
            grid,
            board_state: BoardState {
                tiles: [Option::None; NUMBER_OF_TILES],
                current_player: PlayerSide::Bottom,
                top_pawns: PawnArray::new(),
                bottom_pawns: PawnArray::new(),
            },
            was_pressed: false,
            is_pressed: false,
            is_undo: false,
            hovered_tile: -1,
            prev_mouse_position: Vec2::new(-1_f32, -1_f32),
            selected_pawn: -1,
            possible_plays: Vec::new(),
            top_player_pawn: Pawn {player: PlayerSide::Top, table_index: 0},
            bottom_player_pawn: Pawn{player: PlayerSide::Bottom, table_index: 0},
            top_pawn_count: 3,
            bottom_pawn_count: 3,
            previous_states: Vec::new(),
            ai_timer: -1_f64,
        };

        game.board_state.add_pawn(TileCoord{x:3, y: 0}, PlayerSide::Top);
        game.board_state.add_pawn(TileCoord{x:4, y: 0}, PlayerSide::Top);
        game.board_state.add_pawn(TileCoord{x:5, y: 0}, PlayerSide::Top);

        game.board_state.add_pawn(TileCoord{x:3, y: 3}, PlayerSide::Bottom);
        game.board_state.add_pawn(TileCoord{x:4, y: 4}, PlayerSide::Bottom);
        game.board_state.add_pawn(TileCoord{x:5, y: 3}, PlayerSide::Bottom);

        println!("{0}", game.board_state.top_pawns.count);

        return game;
    }

    #[allow(dead_code)]
    fn draw_tile_indexes(&self, ctx: &mut Context) {
        for index in 0..self.grid.tiles.len(){
            let label = graphics::Text::new(index.to_string());
            let hf = Vec2::new(label.width(&ctx)/ 2_f32, label.height(&ctx) / 2_f32);
            graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + self.grid.tiles[index].position() - hf)).unwrap();
        }
    }

    fn unselect_pawn(&mut self) {
        self.selected_pawn = -1;
        self.possible_plays.clear();
    }
}

impl ggez::event::EventHandler<GameError> for InGameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_position = input::mouse::position(ctx);
        let mouse_position = Vec2::new(mouse_position.x, mouse_position.y);

        if mouse_position != self.prev_mouse_position
        {
            self.hovered_tile = self.grid.get_tile_at(mouse_position);
        }

        self.prev_mouse_position = mouse_position;
        self.was_pressed = self.is_pressed;
        self.is_pressed = input::mouse::button_pressed(ctx, event::MouseButton::Left);
        
        if self.board_state.current_player == PlayerSide::Top {
            if self.ai_timer > 0_f64 {
                let delta = timer::duration_to_f64(timer::delta(ctx));
                self.ai_timer = self.ai_timer - delta;
                if self.ai_timer <= 0_f64 {
                    /*  
                    match Brain::search_best_play(&self.board_state, 1, &self.grid) {
                        Some(best_play) => {
                            self.previous_states.push(self.board_state.clone());
                            self.board_state  = self.board_state.make_move(best_play.0, best_play.1);
                        },

                        None => {}
                    }

                    self.board_state.current_player = self.board_state.current_player.reverse();
                    */
                }
            }

            // return Ok(());
        }

        if !self.was_pressed && self.is_pressed {
            if self.hovered_tile > -1 {
                if self.selected_pawn < 0 {
                    match &mut self.board_state.tiles[self.hovered_tile as usize] {
                        Some(pawn) => {
                            if self.board_state.current_player == pawn.player {
                                self.selected_pawn = self.hovered_tile;
                                let player_side = pawn.player;
                                self.possible_plays = self.board_state.get_possible_plays(self.hovered_tile as usize, player_side, &self.grid);
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
                        
                        match self.board_state.tiles[self.hovered_tile as usize] {
                            Some(pawn) => {
                                match pawn.player {
                                    PlayerSide::Top => { self.top_pawn_count = self.top_pawn_count - 1;},
                                    PlayerSide::Bottom => { self.bottom_pawn_count = self.bottom_pawn_count - 1;},
                                }
                            },
                            None => {},
                        }

                        self.previous_states.push(self.board_state.clone());
                        self.board_state  = self.board_state.make_move(source_index, self.hovered_tile as usize);
                        self.board_state.current_player = self.board_state.current_player.reverse();

                        self.ai_timer = 2_f64;
                    }
                    else {
                        self.unselect_pawn();
                        match &mut self.board_state.tiles[self.hovered_tile as usize] {
                            Some(pawn) => {
                                if self.board_state.current_player == pawn.player {
                                    self.selected_pawn = self.hovered_tile;
                                    let player_side = pawn.player;
                                    self.possible_plays = self.board_state.get_possible_plays(self.hovered_tile as usize, player_side, &self.grid);
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

        let undo = input::keyboard::is_key_pressed(ctx, event::KeyCode::Z) && input::keyboard::is_mod_active(ctx, event::KeyMods::CTRL);
        if undo && !self.is_undo {
            match self.previous_states.pop() {
                Some(state) => {self.board_state = state;}
                None => {println!("History Empty");}
            }
        }

        self.is_undo = undo;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let mesh_builder = &mut graphics::MeshBuilder::new();
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

            tile.build_mesh(style ,mesh_builder);
        }

        for index in 0..NUMBER_OF_TILES {
            if let Some(pawn) = self.board_state.tiles[index] {
                pawn.draw(mesh_builder, self.grid.tiles[index].position(), self.grid.scale * 0.4, self.selected_pawn == index as isize);
            }
        }
        
        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.grid.position))?; 

        let label = graphics::Text::new("Current player : ");
        graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + Vec2::new(self.grid.width / 2. - label.width(ctx), -65.))).unwrap();
        let mesh_builder = &mut graphics::MeshBuilder::new();
        let current_pawn = match self.board_state.current_player {PlayerSide::Bottom => self.bottom_player_pawn, PlayerSide::Top => self.top_player_pawn};
        current_pawn.draw(mesh_builder, self.grid.position + Vec2::new(self.grid.width / 2. + 15., -60.), self.grid.scale * 0.4, false);
        let mesh =  mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx,&mesh, graphics::DrawParam::default()).unwrap();

        if self.hovered_tile > -1 {
            let coord = Grid::get_coord_from_index(self.hovered_tile as usize);
            let label = format!("[{},{}] = {}", coord.x, coord.y, self.hovered_tile);
            let label = graphics::Text::new(label);
            graphics::draw(ctx, &label, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::BLACK);
        let winning_label = graphics::Text::new("Winner :");
        graphics::draw(ctx, &winning_label, graphics::DrawParam::default().dest(Vec2::new(250. - winning_label.width(ctx), 250. - winning_label.height(ctx) / 2.)))?;
        let mesh_builder =  &mut graphics::MeshBuilder::new();
        self.winner_pawn.draw(mesh_builder, Vec2::new(275., 250.), 20., false);
        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        match self {
            GameState::InGame(state) => state.update(ctx),
            GameState::GameOver(state) => state.update(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        match self {
            GameState::InGame(state) => state.draw(ctx),
            GameState::GameOver(state) => state.draw(ctx),
        }
    }
}

impl Game {
    pub fn new(grid_position: Vec2) -> Game {

        let grid = Grid::new(0.3, grid_position, 40., 5.);
        let in_game_state = InGameState::new(grid);
        let game = Game {game_state: GameState::InGame(in_game_state)};
        return game;
    }
}

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.game_state.update(ctx)?;
        if let GameState::InGame(state) = &self.game_state {
            if state.top_pawn_count == 0 {
                self.game_state = GameState::GameOver(GameOverState{winner_pawn: state.bottom_player_pawn});
            }
            else if state.bottom_pawn_count == 0 {
                self.game_state = GameState::GameOver(GameOverState{winner_pawn: state.top_player_pawn});
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.game_state.draw(ctx)
    }
}