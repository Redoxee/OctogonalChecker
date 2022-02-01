use ggez::{*, graphics::MeshBuilder};
use glam::Vec2;
use std::f32::consts::*;

struct OctoCell {
    verts : [Vec2; 8],
    inner_verts : [Vec2; 8],
}

impl OctoCell {
    fn new(position: Vec2, size: f32, thickness: f32) -> OctoCell {
        let cos = FRAC_PI_8.cos();
        let sin = FRAC_PI_8.sin();
        let size = size * 0.5;
        let inner_size = size - thickness / 2.;
        let cell = OctoCell{
                verts:[
                    Vec2::new(cos, sin) * size + position,
                    Vec2::new(sin, cos) * size + position,
                    Vec2::new(-sin, cos) * size + position,
                    Vec2::new(-cos, sin) * size + position,
                    Vec2::new(-cos, -sin) * size + position,
                    Vec2::new(-sin, -cos) * size + position,
                    Vec2::new(sin, -cos) * size + position,
                    Vec2::new(cos, -sin) * size + position,
                ],

                inner_verts :[
                    Vec2::new(cos, sin) * inner_size + position,
                    Vec2::new(sin, cos) * inner_size + position,
                    Vec2::new(-sin, cos) * inner_size + position,
                    Vec2::new(-cos, sin) * inner_size + position,
                    Vec2::new(-cos, -sin) * inner_size + position,
                    Vec2::new(-sin, -cos) * inner_size + position,
                    Vec2::new(sin, -cos) * inner_size + position,
                    Vec2::new(cos, -sin) * inner_size + position,
                ],
        };

        cell
    }
}

struct QuadCell {
    verts : [Vec2; 4],
    inner_verts : [Vec2; 4],
}

impl QuadCell {
    fn new(position: Vec2, size: f32, thickness: f32) -> QuadCell {
        let size = size / 2.;
        let thickness = thickness / 2.;
        let cell= QuadCell{
            verts: [
                Vec2::new(0., -size),
                Vec2::new(size, 0.),
                Vec2::new(0., size),
                Vec2::new(-size, 0.),
            ],
            inner_verts:[
                Vec2::new(0., -size + thickness),
                Vec2::new(size - thickness, 0.),
                Vec2::new(0., size - thickness),
                Vec2::new(-size + thickness, 0.),
            ]
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
        match self {
            Cell::Octogone(octo_cell) => {
                for vert in octo_cell.inner_verts {
                    vertices.push(mint::Point2{x:vert.x, y:vert.y});
                }

                mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &vertices, graphics::Color::WHITE).unwrap();
            },

            Cell::Quad(quad_cell) =>{
                for vert in quad_cell.inner_verts {
                    vertices.push(mint::Point2{x:vert.x, y:vert.y});
                }

                mesh_builder.polygon(graphics::DrawMode::Fill(graphics::FillOptions::default()), &vertices, graphics::Color::RED).unwrap();
            }
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
}

impl Grid{
    fn new(octogone_side: u32, scale: f32, thickness: f32) -> Grid{
        let octo_row = octogone_side;

        let mut grid = Grid{
            cells: Vec::new(),
        };

        let s = 1. / 3.;

        for y_index in 0..octo_row {
            for x_index in 0..octo_row {
                let position = Vec2::new((x_index * 3) as f32, (y_index * 3) as f32) * scale;
                grid.cells.push(Cell::Octogone(OctoCell::new(position, 3. * scale, thickness)));
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
}

impl Grid
{
    fn draw(&mut self, ctx: &mut Context, position: Vec2) -> GameResult {
        
        let mesh = self.build_mesh(ctx);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(position))?;

        Ok(())
    }
}

struct Game {
    grid: Grid,
}

impl ggez::event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
  
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        self.grid.draw(ctx, Vec2::new(30., 30.))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let game_instance = Game {
        grid: Grid::new(8, 25., 3.),
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("OctoChess", "AntonMakesGames")
    .default_conf(c)
    .build()
    .unwrap();

    event::run(ctx, event_loop, game_instance);
}
