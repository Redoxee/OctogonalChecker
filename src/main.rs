use ggez::{*, graphics::MeshBuilder};
use glam::Vec2;

struct OctoCell {
    verts : [Vec2; 8],
    inner_verts : [Vec2; 8],
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
        };

        cell
    }
}

struct QuadCell {
    verts : [Vec2; 4],
    inner_verts : [Vec2; 4],
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
}

struct Grid {
    cells: Vec<Cell>,
    scale: f32,
    octogon_ratio: f32,
}

impl Grid{
    fn new(side_number: u32, octogon_ratio: f32, scale: f32, thickness: f32) -> Grid{

        let mut grid = Grid{
            cells: Vec::new(),
            octogon_ratio,
            scale,
        };

        let half_cell_gap = scale;
        let cell_gap = half_cell_gap * 2.;
        let octo_delta = Vec2::new(half_cell_gap, half_cell_gap);

        for y_index in 0..=side_number {
            for x_index in 0..=side_number {
                let position = Vec2::new((x_index) as f32, (y_index) as f32) * cell_gap;
                grid.cells.push(Cell::Quad(QuadCell::new(position, octogon_ratio, scale, thickness)));
            }
        }
        


        for y_index in 0..=side_number {
            for x_index in 0..=side_number {
                let position = Vec2::new((x_index) as f32, (y_index) as f32) * cell_gap;
                if x_index < side_number && y_index < side_number{
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
  
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let delta = self.grid.octogon_ratio * self.grid.scale * 2. + 10.;
        self.grid.draw(ctx, Vec2::new(delta, delta))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let game_instance = Game {
        grid: Grid::new(8, 0.3, 40., 5.),
    };

    let mut c = conf::Conf::new();
    c.window_mode.width = 920_f32;
    c.window_mode.height = 920_f32;
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
