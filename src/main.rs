use std::fmt::Debug;

use ggez::{*, graphics::MeshBuilder};
use glam::*;

struct OctoCell {
    verts : [Vec2; 8],
    inner_verts : [Vec2; 8],
    position : Vec2,
}

impl OctoCell {
    fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> OctoCell {
        let half = octogon_ratio * size;

        let inner_size = size - thickness / 2.;
        let inner_half = octogon_ratio * (size - thickness/2.);
        let cell = OctoCell{
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

        cell
    }
}

struct QuadCell {
    verts : [Vec2; 4],
    inner_verts : [Vec2; 4],
    position : Vec2,
}

impl QuadCell {
    fn new(position: Vec2, octogon_ratio: f32, size: f32, thickness: f32) -> QuadCell {
        let size = size * (1. - octogon_ratio);
        let thickness = thickness / 2.;
        let cell= QuadCell{
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

        cell
    }
}

enum Cell {
    Octogone(OctoCell),
    Quad(QuadCell),
}

impl Cell {
    fn build_mesh(&self, mesh_builder: &mut MeshBuilder) {
        let mut vertices = Vec::new();
        let octo_color = graphics::Color::new(1., 1., 1., 0.5);
        match self {
            Cell::Octogone(octo_cell) => {
                for vert in octo_cell.inner_verts {
                    vertices.push(mint::Point2{x:vert.x, y:vert.y});
                }

                mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &vertices, octo_color).unwrap();
                vertices.clear();
            },

            Cell::Quad(quad_cell) =>{
                for vert in quad_cell.inner_verts {
                    vertices.push(mint::Point2{x:vert.x, y:vert.y});
                }

                mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &vertices, graphics::Color::RED).unwrap();
                vertices.clear();
            }
        }
    }

    fn contain_position(&self, position: &Vec2) -> bool{
        let mut angle = 0_f32;
        match self {
            Cell::Octogone(cell) =>
            {
                for vertice in cell.verts {
                    if *position == vertice {
                        return true
                    }

                    angle += position.angle_between(vertice);
                }
            }

            Cell::Quad(cell) =>
            {
                for vertice in cell.verts {
                    if *position == vertice {
                        return true
                    }

                    angle += position.angle_between(vertice);
                }
            }
        }

        return angle != 0_f32;
    }
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
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    position: Vec2,
    scale: f32,
    bounding_box: BoundingBox,
}

#[derive(Debug)]
struct CellCoord{
    x:i32,
    y:i32,
}

enum IsCell
{
    None,
    Yes(CellCoord),
}

impl Debug for IsCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            IsCell::Yes(cell)=>
                f.debug_struct("Point")
                .field("x", &cell.x)
                .field("y", &cell.y)
                .finish(),
                IsCell::None=>f.write_str("[oob]")
        }
    }
}

impl Grid{
    fn new(octogon_on_side: u32, octogon_ratio: f32, position: Vec2, scale: f32, thickness: f32) -> Grid{
        let height = octogon_on_side;
        let cell_on_side = octogon_on_side * 2 + 1;
        let bb_scale = scale * (cell_on_side + 1) as f32;
        let mut grid = Grid{
            cells: Vec::new(),
            width: cell_on_side,
            height: octogon_on_side + 1,
            position,
            scale,
            bounding_box: BoundingBox::new(position.x - scale, position.y - scale, bb_scale, bb_scale)
        };

        let half_cell_gap = scale;
        let cell_gap = half_cell_gap * 2.;
        let octo_delta = Vec2::new(half_cell_gap, half_cell_gap);

        for y_index in 0..=octogon_on_side {
            for x_index in 0..=octogon_on_side {
                let position = Vec2::new((x_index) as f32, (y_index) as f32) * cell_gap;
                grid.cells.push(Cell::Quad(QuadCell::new(position, octogon_ratio, scale, thickness)));

                if x_index < octogon_on_side && y_index < octogon_on_side{
                    grid.cells.push(Cell::Octogone(OctoCell::new(position + octo_delta, octogon_ratio, scale, thickness)));
                }
            }
        }

        grid
    }

    fn build_mesh(&self, ctx: &mut Context) -> graphics::Mesh{
        let mesh_builder = &mut graphics::MeshBuilder::new();
        for cell in &self.cells
        {
            cell.build_mesh(mesh_builder);
        }
        
        mesh_builder.build(ctx).unwrap()
    }

    fn get_index_from_coord(&self, coord: &CellCoord) -> Option<usize>
    {
        let width = self.width as i32;
        let height = self.height as i32;
        if coord.x < 0 || coord.y < 0 || coord.x >= width || coord.y >= height {
            return Option::None
        }

        if coord.y < height - 1 {
            return Option::Some((coord.y * width + coord.x) as usize)
        }
        else if coord.x % 2 == 0{
            return Option::Some((coord.y * width + (coord.x) / 2) as usize)
        }
        else {
            return Option::None
        }
    }

    fn get_cell_at(&self, position: Vec2) -> IsCell{
        if !self.bounding_box.is_in(&position) {
            return IsCell::None
        }

        let coord = position - self.position;
        let base_x = (coord.x / self.scale / 2_f32).floor() as i32;
        let base_y = (coord.y / self.scale / 2_f32).floor() as i32;
        println!("Pressed big square [{},{}]", base_x, base_y);

        let mut possible_coord = Vec::new();
        possible_coord.push(CellCoord{x: base_x * 2, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2 + 1, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2 + 2, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2, y: base_y + 1});
        possible_coord.push(CellCoord{x: base_x * 2 + 2, y: base_y + 1});

        for coord in possible_coord{
            println!("[{},{}] = {}", coord.x, coord.y, match self.get_index_from_coord(&coord) {Some(index) => index.to_string(), None => String::from("Oob")});
        }

        let coord = Vec2::new(0_f32 , 0_f32);
        
        return IsCell::Yes(CellCoord{x: coord.x.floor() as i32, y: coord.y.floor() as i32})
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        let mesh = self.build_mesh(ctx);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.position))?;
        for index in 0..self.cells.len(){
            let label = graphics::Text::new(index.to_string());
            let hf = Vec2::new(label.width(&ctx)/ 2_f32, label.height(&ctx) / 2_f32);
            match &self.cells[index] {
                Cell::Octogone(cell) => 
                {
                    graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.position + cell.position - hf))?;
                },

                Cell::Quad(cell) =>
                {
                    graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.position + cell.position - hf))?;
                }
            }
        }

        Ok(())
    }
}

struct Game {
    grid: Grid,
    was_pressed: bool,
    is_pressed: bool,
    hovered_cell: IsCell,
}

impl Game {
    fn new(grid : Grid) -> Game{
        Game{
            grid,
            was_pressed: false,
            is_pressed: false,
            hovered_cell: IsCell::None,
        }
    }
}

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_position = input::mouse::position(ctx);
        self.was_pressed = self.is_pressed;
        self.is_pressed = input::mouse::button_pressed(ctx, event::MouseButton::Left);
        let mouse_position = Vec2::new(mouse_position.x, mouse_position.y);
        if !self.was_pressed && self.is_pressed {
            println!("PRESS");
            self.hovered_cell = self.grid.get_cell_at(mouse_position);
        }

        Ok(())
    }
  
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        self.grid.draw(ctx)?;

        if self.is_pressed
        {
            match &self.hovered_cell{
                IsCell::None=>{},
                IsCell::Yes(coord)=>{
                    let label = format!("[{},{}]", coord.x, coord.y);
                    let label = graphics::Text::new(label);
                    graphics::draw(ctx, &label, graphics::DrawParam::default())?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main(){

    let grid_position = Vec2::new(100., 100.);
    let game_instance = Game::new(
        Grid::new(3, 0.3, grid_position, 40., 5.),
    );

    let mut c = conf::Conf::new();
    c.window_mode.width = 720_f32;
    c.window_mode.height = 720_f32;
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
