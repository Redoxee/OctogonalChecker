use ggez::{*, graphics::MeshBuilder};
use glam::*;

use crate::shape_style::*;
use crate::tiles::*;
use crate::utils::*;
use crate::grid::*;

#[derive(Clone, Copy)]
pub struct Pawn {
    player : PlayerSide,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum PlayerSide {
    Bottom,
    Top,
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
    hovered_tile: isize,
    board_state : [Option<Pawn>; NUMBER_OF_TILES],
    selected_pawn : isize,
    possible_plays: Vec<usize>,
    current_player : PlayerSide,
    top_player_pawn : Pawn,
    bottom_player_pawn : Pawn,
    top_pawn_count : i8,
    bottom_pawn_count : i8,
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

impl InGameState {
    fn new(grid : Grid) -> InGameState{
        let mut game = InGameState{
            grid,
            was_pressed: false,
            is_pressed: false,
            hovered_tile: -1,
            prev_mouse_position: Vec2::new(-1_f32, -1_f32),
            board_state: [Option::None; NUMBER_OF_TILES],
            selected_pawn: -1,
            possible_plays: Vec::new(),
            top_player_pawn: Pawn {player: PlayerSide::Top},
            bottom_player_pawn: Pawn{player: PlayerSide::Bottom},
            current_player: PlayerSide::Bottom,
            top_pawn_count: 3,
            bottom_pawn_count: 3,
        };

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 3, y: 0})] = Some(Pawn{
            player: PlayerSide::Top,
        });

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 4, y: 0})] = Some(Pawn{
            player: PlayerSide::Top,
        });

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 5, y: 0})] = Some(Pawn{
            player: PlayerSide::Top,
        });

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 3, y: 3})] = Some(Pawn{
            player: PlayerSide::Bottom,
        });

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 4, y: 4})] = Some(Pawn{
            player: PlayerSide::Bottom,
        });

        game.board_state[game.grid.get_index_from_coord_unsafe(TileCoord{x: 5, y: 3})] = Some(Pawn{
            player: PlayerSide::Bottom,
        });

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

    fn get_possible_plays(&self, tile_index: usize, player_side: PlayerSide) -> Vec<usize>{
        let coord = self.grid.get_coord_from_index(tile_index);
        let mut possible_plays = Vec::new();
        match self.grid[coord] {
            GridTile::Quad(_) => {
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y - 1}) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y - 1}) {possible_plays.push(index)};
            },

            GridTile::Octo(_) => {
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x + 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x - 2, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x    , y: coord.y - 1}) {possible_plays.push(index)};

                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y }) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x + 1, y: coord.y + 1}) {possible_plays.push(index)};
                if let Some(index) = self.grid.get_index_from_coord(TileCoord{ x: coord.x - 1, y: coord.y + 1}) {possible_plays.push(index)};
            },

            GridTile::None => panic!(),
        }

        possible_plays.retain(|&index| match self.board_state[index] {
            Some(pawn) => { pawn.player != player_side },
            None => true
        });

        return possible_plays;
    }
    
    fn make_move(board: [Option<Pawn>; NUMBER_OF_TILES], source_index: usize, play_index: usize) -> [Option<Pawn>; NUMBER_OF_TILES]{
        let mut board = board.clone();
        let pawn = board[source_index];
        board[play_index] = pawn;
        board[source_index] = Option::None;
        return board;
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
        if !self.was_pressed && self.is_pressed {
            if self.hovered_tile > -1 {
                if self.selected_pawn < 0 {
                    match &mut self.board_state[self.hovered_tile as usize] {
                        Some(pawn) => {
                            if self.current_player == pawn.player {
                                self.selected_pawn = self.hovered_tile;
                                let player_side = pawn.player;
                                self.possible_plays = self.get_possible_plays(self.hovered_tile as usize, player_side);
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
                        
                        match self.board_state[self.hovered_tile as usize] {
                            Some(pawn) => {
                                match pawn.player {
                                    PlayerSide::Top => { self.top_pawn_count = self.top_pawn_count - 1;},
                                    PlayerSide::Bottom => { self.bottom_pawn_count = self.bottom_pawn_count - 1;},
                                }
                            },
                            None => {},
                        }

                        self.board_state = InGameState::make_move(self.board_state, source_index, self.hovered_tile as usize);

                        if let Some(pawn) = self.board_state[self.hovered_tile as usize] {
                            match pawn.player {
                                PlayerSide::Top => self.current_player = PlayerSide::Bottom,
                                PlayerSide::Bottom => self.current_player = PlayerSide::Top,
                            }
                        }
                    }
                    else {
                        self.unselect_pawn();
                        match &mut self.board_state[self.hovered_tile as usize] {
                            Some(pawn) => {
                                if self.current_player == pawn.player {
                                    self.selected_pawn = self.hovered_tile;
                                    let player_side = pawn.player;
                                    self.possible_plays = self.get_possible_plays(self.hovered_tile as usize, player_side);
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

        for index in 0..self.board_state.len() {
            if let Some(pawn) = self.board_state[index] {
                pawn.draw(mesh_builder, self.grid.tiles[index].position(), self.grid.scale * 0.4, self.selected_pawn == index as isize);
            }
        }
        
        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.grid.position))?; 


        
        let label = graphics::Text::new("Current player : ");
        graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + Vec2::new(self.grid.width / 2. - label.width(ctx), -65.))).unwrap();
        let mesh_builder = &mut graphics::MeshBuilder::new();
        let current_pawn = match self.current_player {PlayerSide::Bottom => self.bottom_player_pawn, PlayerSide::Top => self.top_player_pawn};
        current_pawn.draw(mesh_builder, self.grid.position + Vec2::new(self.grid.width / 2. + 15., -60.), self.grid.scale * 0.4, false);
        let mesh =  mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx,&mesh, graphics::DrawParam::default()).unwrap();

        if self.hovered_tile > -1 {
            let coord = self.grid.get_coord_from_index(self.hovered_tile as usize);
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