use ggez::{*, graphics::MeshBuilder};
use glam::*;
use::std::ops;

const GRID_SIDE: usize = 4;
const NUMBER_OF_TILES: usize = (GRID_SIDE * 2 + 1)  * GRID_SIDE + GRID_SIDE + 1;

#[derive(Clone, Copy)]
struct OctoTile {
    verts : [Vec2; 8],
    inner_verts : [Vec2; 8],
    position : Vec2,
}

#[derive(Clone, Copy)]
struct QuadTile {
    verts : [Vec2; 4],
    inner_verts : [Vec2; 4],
    position : Vec2,
}

trait Tile {
    fn build_mesh(&self, style: TileStyle,mesh_builder: &mut MeshBuilder);
    fn contain_position(&self, position: &Vec2) -> bool;
    fn position(&self) -> Vec2;
}

impl OctoTile {
    fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> OctoTile {
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
    fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> QuadTile {
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

impl Tile for OctoTile{
    fn build_mesh(&self, style: TileStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            TileStyle::Base => graphics::Color::new(0.6, 0.6, 0.6, 1_f32),
            TileStyle::Highlight => graphics::Color::new(0.3, 0.4, 0.5, 1_f32),
            TileStyle::Hovered => graphics::Color::new(0.8, 0.8, 0.8, 1_f32),
            TileStyle::Press => graphics::Color::new(0.9, 0.9, 0.9, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();

        match style {
            TileStyle::Highlight => {
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

impl Tile for QuadTile{
    fn build_mesh(&self, style: TileStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            TileStyle::Base => graphics::Color::new(0.7, 0., 0., 1_f32),
            TileStyle::Highlight => graphics::Color::new(0.1, 0., 0.3, 1_f32),
            TileStyle::Hovered => graphics::Color::new(0.8, 0.3, 0.3, 1_f32),
            TileStyle::Press => graphics::Color::new(0.9, 0.5, 0.5, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();
        match style {
            TileStyle::Highlight => {
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

#[derive(Clone, Copy)]
enum GridTile {
    Quad(QuadTile),
    Octo(OctoTile),
    None,
}

impl Tile for GridTile {
    fn build_mesh(&self, style: TileStyle,mesh_builder: &mut MeshBuilder) {
        match self {
            GridTile::Quad(inner_tile) => inner_tile.build_mesh(style, mesh_builder),
            GridTile::Octo(inner_tile) => inner_tile.build_mesh(style, mesh_builder),
            GridTile::None => panic!()
        }
    }

    fn contain_position(&self, position: &Vec2) -> bool {
        match self {
            GridTile::Quad(inner_tile) => inner_tile.contain_position(position),
            GridTile::Octo(inner_tile) => inner_tile.contain_position(position),
            GridTile::None => panic!()
        }
    }

    fn position(&self) -> Vec2 {
        match self {
            GridTile::Quad(inner_tile) => inner_tile.position(),
            GridTile::Octo(inner_tile) => inner_tile.position(),
            GridTile::None => panic!()
        }
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

enum TileStyle {
    Base,
    Hovered,
    Highlight,
    Press,
}

struct BoundingBox{
    x: f32,
    right: f32,
    top: f32,
    y: f32,
}

impl BoundingBox{
    fn new(x: f32, y: f32, width: f32, height:f32) -> BoundingBox{
        BoundingBox{
            x,
            y,
            right: x + width,
            top: y + height,
        }
    }

    fn is_in(&self, position: &Vec2) -> bool{
        position.x >= self.x && position.y >= self.y && position.x <= self.right && position.y <= self.top
    }
}

struct Grid {
    tiles: [GridTile; NUMBER_OF_TILES],
    tiles_on_row: i32,
    tiles_on_col: i32,
    position: Vec2,
    scale: f32,
    width: f32,
    bounding_box: BoundingBox,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
struct TileCoord{
    x:i32,
    y:i32,
}

impl ops::Add<TileCoord> for TileCoord{
    type Output = TileCoord;

    fn add(self, rhs: TileCoord) -> TileCoord {
        TileCoord{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Grid{
    fn new(octogon_ratio: f32, position: Vec2, scale: f32, thickness: f32) -> Grid{
        let tile_on_side = GRID_SIDE * 2 + 1;
        let bb_scale = scale * (tile_on_side + 1) as f32;
        let mut grid = Grid{
            tiles: [GridTile::None; NUMBER_OF_TILES],
            tiles_on_row: tile_on_side as i32,
            tiles_on_col: GRID_SIDE as i32 + 1,
            position,
            scale,
            bounding_box: BoundingBox::new(position.x - scale, position.y - scale, bb_scale, bb_scale),
            width: GRID_SIDE as f32 * scale * 2.,
        };

        let half_tile_gap = scale;
        let tile_gap = half_tile_gap * 2.;
        let octo_delta = Vec2::new(half_tile_gap, half_tile_gap);

        let mut array_index = 0;
        for y_index in 0..=GRID_SIDE {
            for x_index in 0..=GRID_SIDE {
                let position = Vec2::new(x_index as f32, y_index as f32) * tile_gap;
                grid.tiles[array_index] = GridTile::Quad(QuadTile::new(position, octogon_ratio, scale, thickness));
                array_index += 1;

                if x_index < GRID_SIDE && y_index < GRID_SIDE{
                    grid.tiles[array_index] = GridTile::Octo(OctoTile::new(position + octo_delta, octogon_ratio, scale, thickness));
                    array_index += 1;
                }
            }
        }

        grid
    }

    fn get_index_from_coord(&self, coord: TileCoord) -> Option<usize> {
        let width = self.tiles_on_row;
        let height = self.tiles_on_col;
        if coord.x < 0 || coord.y < 0 || coord.x >= width || coord.y >= height {
            return Option::None
        }

        if coord.y < height - 1 || coord.x % 2 == 0{
            return Some(self.get_index_from_coord_unsafe(coord))
        }
        else {
            return Option::None
        }
    }

    fn get_index_from_coord_unsafe(&self, coord: TileCoord) -> usize {
        let width = self.tiles_on_row as i32;
        let height = self.tiles_on_col as i32;
        
        if coord.y < height - 1 {
            return (coord.y * width + coord.x) as usize
        }
        else
        {
            return (coord.y * width + (coord.x) / 2) as usize
        }
    }

    fn get_coord_from_index(&self, index : usize) -> TileCoord {
        let index = index as i32;
        let width = self.tiles_on_row as i32;
        let height = self.tiles_on_col as i32;
        let mut result = TileCoord{x: index % width, y: index / width}; 
        if result.y == height - 1 {
            result.x = result.x * 2;
        }

        return result;
    }

    fn get_tile_at(&self, position: Vec2) -> isize{
        if !self.bounding_box.is_in(&position) {
            return -1
        }

        let coord = position - self.position;
        let base_x = (coord.x / self.scale / 2_f32).floor() as i32;
        let base_y = (coord.y / self.scale / 2_f32).floor() as i32;

        let mut possible_coord = Vec::new();
        possible_coord.push(TileCoord{x: base_x * 2, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2 + 1, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2 + 2, y: base_y});
        possible_coord.push(TileCoord{x: base_x * 2, y: base_y + 1});
        possible_coord.push(TileCoord{x: base_x * 2 + 2, y: base_y + 1});

        let position = position - self.position;
        for coord in possible_coord {
            match self.get_index_from_coord(coord) {
                Some(index)=> {
                    if self.tiles[index].contain_position(&position) {
                        return index as isize
                    }
                },

                None=> {},
            }
        }
        
        return -1
    }
}

impl ops::Index<TileCoord> for Grid {
    type Output = GridTile;
    
    fn index(&self, index: TileCoord) -> &GridTile {
        let index = self.get_index_from_coord_unsafe(index);
        return &self.tiles[index]
    }
}

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
enum PawnState {
    None,
    Selected,
}

#[derive(Clone, Copy)]
struct Pawn {
    player : PlayerSide,
    state : PawnState,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
enum PlayerSide {
    Bottom,
    Top,
}

impl Pawn {
    fn draw(&self, mesh_builder: &mut MeshBuilder, position: Vec2, scale: f32){
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

        if self.state == PawnState::Selected {
            secondary_color = graphics::Color::YELLOW;
        }

        mesh_builder.circle(graphics::DrawMode::Fill(graphics::FillOptions::default()), position, scale, 0.1, primary_color).unwrap();
        mesh_builder.circle(graphics::DrawMode::Stroke(graphics::StrokeOptions::default().with_line_width(3.)), position, scale, 0.1, secondary_color).unwrap();
    }
}

struct InGameState {
    grid: Grid,
    prev_mouse_position: Vec2,
    was_pressed: bool,
    is_pressed: bool,
    hovered_tile: isize,
    pawns : [Option<Pawn>; NUMBER_OF_TILES],
    selected_pawn : isize,
    possible_plays: Vec<usize>,
    current_player : PlayerSide,
    top_player_pawn : Pawn,
    bottom_player_pawn : Pawn,
    top_pawn_count : i8,
    bottom_pawn_count : i8,
}

impl InGameState {
    fn new(grid : Grid) -> InGameState{
        let mut game = InGameState{
            grid,
            was_pressed: false,
            is_pressed: false,
            hovered_tile: -1,
            prev_mouse_position: Vec2::new(-1_f32, -1_f32),
            pawns: [Option::None; NUMBER_OF_TILES],
            selected_pawn: -1,
            possible_plays: Vec::new(),
            top_player_pawn: Pawn {player: PlayerSide::Top, state: PawnState::None},
            bottom_player_pawn: Pawn{player: PlayerSide::Bottom, state: PawnState::None},
            current_player: PlayerSide::Bottom,
            top_pawn_count: 3,
            bottom_pawn_count: 3,
        };

        game.add_pawn(PlayerSide::Top, TileCoord{x: 3, y: 0});
        game.add_pawn(PlayerSide::Top, TileCoord{x: 4, y: 0});
        game.add_pawn(PlayerSide::Top, TileCoord{x: 5, y: 0});

        game.add_pawn(PlayerSide::Bottom, TileCoord{x: 3, y: 3});
        game.add_pawn(PlayerSide::Bottom, TileCoord{x: 4, y: 4});
        game.add_pawn(PlayerSide::Bottom, TileCoord{x: 5, y: 3});

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
        match &mut self.pawns[self.selected_pawn as usize] {
            Some(pawn) => {
                pawn.state = PawnState::None;
                self.selected_pawn = -1;
                self.possible_plays.clear();
            },

            None => { panic!() }
        }
    }

    fn add_pawn(&mut self, side: PlayerSide, coord: TileCoord) {
        self.pawns[self.grid.get_index_from_coord_unsafe(coord)] = Some(Pawn{
            state: PawnState::None,
            player: side,
        });
    }

    fn get_possible_plays(&self, index: usize, player_side: PlayerSide) -> Vec<usize>{
        let coord = self.grid.get_coord_from_index(index);
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

        possible_plays.retain(|&index| match self.pawns[index] {
            Some(pawn) => { pawn.player != player_side },
            None => true
        });

        return possible_plays;
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
                    match &mut self.pawns[self.hovered_tile as usize] {
                        Some(pawn) => {
                            if self.current_player == pawn.player {
                                pawn.state = PawnState::Selected;
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
                        
                        match self.pawns[self.hovered_tile as usize] {
                            Some(pawn) => {
                                match pawn.player {
                                    PlayerSide::Top => { self.top_pawn_count = self.top_pawn_count - 1;},
                                    PlayerSide::Bottom => { self.bottom_pawn_count = self.bottom_pawn_count - 1;},
                                }
                            },
                            None => {},
                        }

                        self.pawns[self.hovered_tile as usize] = self.pawns[source_index];
                        self.pawns[source_index] = None;

                        if let Some(pawn) = self.pawns[self.hovered_tile as usize] {
                            match pawn.player {
                                PlayerSide::Top => self.current_player = PlayerSide::Bottom,
                                PlayerSide::Bottom => self.current_player = PlayerSide::Top,
                            }
                        }
                    }
                    else {
                        self.unselect_pawn();
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
            let mut style = TileStyle::Base;
            if self.hovered_tile == index as isize {
                if self.is_pressed {
                    style = TileStyle::Press
                }
                else {
                    style = TileStyle::Hovered
                }
            }
            else if self.possible_plays.contains(&index) {
                style = TileStyle::Highlight;
            }

            tile.build_mesh(style ,mesh_builder);
        }

        for index in 0..self.pawns.len() {
            if let Some(pawn) = self.pawns[index] {
                pawn.draw(mesh_builder, self.grid.tiles[index].position(), self.grid.scale * 0.4);
            }
        }
        
        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.grid.position))?; 


        
        let label = graphics::Text::new("Current player : ");
        graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + Vec2::new(self.grid.width / 2. - label.width(ctx), -65.))).unwrap();
        let mesh_builder = &mut graphics::MeshBuilder::new();
        let current_pawn = match self.current_player {PlayerSide::Bottom => self.bottom_player_pawn, PlayerSide::Top => self.top_player_pawn};
        current_pawn.draw(mesh_builder, self.grid.position + Vec2::new(self.grid.width / 2. + 15., -60.), self.grid.scale * 0.4);
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

struct GameOverState {
    winner_pawn : Pawn,
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
        self.winner_pawn.draw(mesh_builder, Vec2::new(275., 250.), 20.);
        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}

enum GameState {
    InGame(InGameState),
    GameOver(GameOverState),
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

struct Game {
    game_state : GameState,
}


impl Game {
    fn new(grid_position: Vec2) -> Game {

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

fn main(){

    let grid_position = Vec2::new(90., 90.);
    let game_instance = Game::new(grid_position);

    let mut c = conf::Conf::new();
    c.window_mode.width = 500_f32;
    c.window_mode.height = 500_f32;
    let (ctx, event_loop) = ContextBuilder::new("OctoChess", "AntonMakesGames")
    .default_conf(c)
    .window_setup(conf::WindowSetup{
        title:String::from("Octogonal chess"),
        samples: conf::NumSamples::One,
        vsync: true,
        srgb:true,
        icon:"".to_owned(),
    })
    .build()
    .unwrap();

    event::run(ctx, event_loop, game_instance);
}
