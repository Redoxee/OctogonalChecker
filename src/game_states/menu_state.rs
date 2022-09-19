use ggez::{
    *,
};

use crate::game::*;
use crate::shape_style::ShapeStyle;
use crate::ui::*;
use crate::grid::*;
use crate::utils::Shape;
use crate::pawn::*;

pub struct MenuState {
    one_player: Button,
    two_player: Button,

    grid_mesh: graphics::Mesh,
    grid_position: glam::Vec2,
    letters: Vec<(graphics::Text, glam::Vec2)>,
    pawns: Vec<(Pawn, glam::Vec2)>,
}

pub enum MenuOption {
    None,
    OnePlayer,
    TwoPlayer,
}

impl MenuState {
    pub fn new(ctx: &mut ggez::Context) -> MenuState{
        let grid_position = glam::Vec2::new(120., 45.);
        let grid = Grid::new(0.3, grid_position, 60., 5.);

        let style = ShapeStyle::Base;
        let mut mesh_builder = graphics::MeshBuilder::new();
        for index in 0..(TILES_ON_SIDE * 2) {
                let tile = grid.tiles[index];
                tile.build_mesh(style, &mut mesh_builder);
        }

        let mesh = mesh_builder.build(ctx).unwrap();

        let mut letters = Vec::new();
        let font = graphics::Font::default();
        let font_size = 60_f32;
        let font_size = graphics::PxScale {x: font_size, y: font_size};
        letters.push((graphics::Text::new("O").set_font(font, font_size).to_owned(), glam::Vec2::new(165_f32, 77_f32)));
        letters.push((graphics::Text::new("C").set_font(font, font_size).to_owned(), glam::Vec2::new(285_f32, 77_f32)));
        letters.push((graphics::Text::new("T").set_font(font, font_size).to_owned(), glam::Vec2::new(405_f32, 77_f32)));
        letters.push((graphics::Text::new("O").set_font(font, font_size).to_owned(), glam::Vec2::new(525_f32, 77_f32)));

        letters.push((graphics::Text::new("C").set_font(font, font_size).to_owned(), glam::Vec2::new(105_f32, 137_f32)));
        letters.push((graphics::Text::new("H").set_font(font, font_size).to_owned(), glam::Vec2::new(225_f32, 137_f32)));
        letters.push((graphics::Text::new("E").set_font(font, font_size).to_owned(), glam::Vec2::new(345_f32, 137_f32)));
        letters.push((graphics::Text::new("S").set_font(font, font_size).to_owned(), glam::Vec2::new(465_f32, 137_f32)));
        letters.push((graphics::Text::new("S").set_font(font, font_size).to_owned(), glam::Vec2::new(585_f32, 137_f32)));

        let mut pawns = Vec::new();
        pawns.push((Pawn{player: PlayerSide::Top, table_index: 0}, glam::Vec2::new(177_f32, 225_f32)));
        pawns.push((Pawn{player: PlayerSide::Bottom, table_index: 0}, glam::Vec2::new(297_f32, 225_f32)));
        pawns.push((Pawn{player: PlayerSide::Top, table_index: 0}, glam::Vec2::new(417_f32, 225_f32)));
        pawns.push((Pawn{player: PlayerSide::Bottom, table_index: 0}, glam::Vec2::new(537_f32, 225_f32)));

        MenuState {
            one_player: Button::new("1 Player", graphics::Rect::new_i32(200, 450, 150, 60)),
            two_player: Button::new("2 Players", graphics::Rect::new_i32(380, 450, 150, 60)),
            grid_mesh: mesh,
            grid_position,
            letters,
            pawns,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, drawing_context: &mut DrawingContext) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::BLACK);
        self.one_player.draw(ctx)?;
        self.two_player.draw(ctx)?;

        graphics::draw(ctx, &self.grid_mesh, graphics::DrawParam::default().dest(self.grid_position))?; 

        for (letter, position) in &self.letters {
            graphics::draw(ctx,letter, graphics::DrawParam::default().dest(*position))?;
        }

        for (pawn, position) in &self.pawns {
            pawn.draw(drawing_context, ctx, *position, 2_f32, false);
        }

        Ok(())
    }

    pub fn update(&mut self, ctx: &Context) -> Result<MenuOption, GameError> {
        if self.one_player.update(ctx) {
            return Ok( MenuOption::OnePlayer);
        }

        if self.two_player.update(ctx) {
            return Ok( MenuOption::TwoPlayer);
        }

        Ok( MenuOption::None)
    }
}