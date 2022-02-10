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

trait Cell {
    fn build_mesh(&self, style: CellStyle,mesh_builder: &mut MeshBuilder);
    fn contain_position(&self, position: &Vec2) -> bool;
    fn position(&self) -> Vec2;
}

impl Cell for OctoCell{
    fn build_mesh(&self, style: CellStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            CellStyle::Base => graphics::Color::new(0.6, 0.6, 0.6, 1_f32),
            CellStyle::Hovered => graphics::Color::new(0.8, 0.8, 0.8, 1_f32),
            CellStyle::Press => graphics::Color::new(0.9, 0.9, 0.9, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();

        match style {
            CellStyle::Press => {
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


impl Cell for QuadCell{
    fn build_mesh(&self, style: CellStyle,mesh_builder: &mut MeshBuilder) {
        let color = match style {
            CellStyle::Base => graphics::Color::new(0.7, 0., 0., 1_f32),
            CellStyle::Hovered => graphics::Color::new(0.8, 0.3, 0.3, 1_f32),
            CellStyle::Press => graphics::Color::new(0.9, 0.5, 0.5, 1_f32),
        };
        
        mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &self.inner_verts.to_vec(), color).unwrap();
        match style {
            CellStyle::Press => {
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

enum CellStyle {
    Base,
    Hovered,
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
    cells: Vec<Box<dyn Cell>>,
    width: u32,
    height: u32,
    position: Vec2,
    scale: f32,
    bounding_box: BoundingBox,
}

struct CellCoord{
    x:i32,
    y:i32,
}

impl Grid{
    fn new(octogon_on_side: u32, octogon_ratio: f32, position: Vec2, scale: f32, thickness: f32) -> Grid{
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
                grid.cells.push(Box::new(QuadCell::new(position, octogon_ratio, scale, thickness)));

                if x_index < octogon_on_side && y_index < octogon_on_side{
                    grid.cells.push(Box::new(OctoCell::new(position + octo_delta, octogon_ratio, scale, thickness)));
                }
            }
        }

        grid
    }

    fn get_index_from_coord(&self, coord: &CellCoord) -> Option<usize> {
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

    fn get_coord_from_index(&self, index : usize) -> CellCoord {
        CellCoord{x: (index as u32 % self.width) as i32, y: (index as u32 / self.width) as i32}
    }

    fn get_cell_at(&self, position: Vec2) -> Option<usize>{
        if !self.bounding_box.is_in(&position) {
            return None
        }

        let coord = position - self.position;
        let base_x = (coord.x / self.scale / 2_f32).floor() as i32;
        let base_y = (coord.y / self.scale / 2_f32).floor() as i32;

        let mut possible_coord = Vec::new();
        possible_coord.push(CellCoord{x: base_x * 2, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2 + 1, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2 + 2, y: base_y});
        possible_coord.push(CellCoord{x: base_x * 2, y: base_y + 1});
        possible_coord.push(CellCoord{x: base_x * 2 + 2, y: base_y + 1});

        let position = position - self.position;
        for coord in possible_coord{
            match self.get_index_from_coord(&coord) {
                Some(index)=> {
                    if self.cells[index].contain_position(&position) {
                        return Option::Some(index)
                    }
                },

                None=> {},
            }
        }
        
        return Option::None
    }
}

struct Game {
    grid: Grid,
    prev_mouse_position: Vec2,
    was_pressed: bool,
    is_pressed: bool,
    hovered_cell: Option<usize>,
}

impl Game {
    fn new(grid : Grid) -> Game{
        Game{
            grid,
            was_pressed: false,
            is_pressed: false,
            hovered_cell: None,
            prev_mouse_position: Vec2::new(-1_f32, -1_f32),
        }
    }
}

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_position = input::mouse::position(ctx);
        let mouse_position = Vec2::new(mouse_position.x, mouse_position.y);

        if mouse_position != self.prev_mouse_position
        {
            self.hovered_cell = self.grid.get_cell_at(mouse_position);
        }

        self.prev_mouse_position = mouse_position;
        self.was_pressed = self.is_pressed;
        self.is_pressed = input::mouse::button_pressed(ctx, event::MouseButton::Left);
        if !self.was_pressed && self.is_pressed {
            match &self.hovered_cell {
                None => println!("Oob"),
                Some(cell) => {
                    let coord = self.grid.get_coord_from_index(*cell);
                    println!("[{},{}]", coord.x, coord.y);
                }
            }
        }

        Ok(())
    }
  
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let mesh_builder = &mut graphics::MeshBuilder::new();
        for index in 0..self.grid.cells.len()
        {
            let cell = &self.grid.cells[index];
            let mut style = CellStyle::Base;
            match self.hovered_cell {
                Some(hovered_index) => if hovered_index == index {
                    if self.is_pressed {
                        style = CellStyle::Press
                    }
                    else {
                        style = CellStyle::Hovered
                    }
                },
                None => {},
            }

            cell.build_mesh(style ,mesh_builder);
        }

        let mesh = mesh_builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(self.grid.position))?; 
/*
for index in 0..self.grid.cells.len(){
    let label = graphics::Text::new(index.to_string());
    let hf = Vec2::new(label.width(&ctx)/ 2_f32, label.height(&ctx) / 2_f32);
    graphics::draw(ctx, &label, graphics::DrawParam::default().dest(self.grid.position + self.grid.cells[index].position() - hf))?;
}
*/

        if self.is_pressed
        {
            match &self.hovered_cell{
                None=>{},
                Some(index)=>{
                    let coord = self.grid.get_coord_from_index(*index);
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

    let grid_position = Vec2::new(120., 120.);
    let game_instance = Game::new(
        Grid::new(6, 0.3, grid_position, 40., 5.),
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
