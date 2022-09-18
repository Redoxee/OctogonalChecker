use ggez::{*,
    graphics::{
       self, 
   },

   Context
};

use glam::*;

use crate::{
        pawn::*,
        DrawingContext,
    };


pub struct GameOverState {
    pub winner_pawn : Pawn,
}

impl GameOverState {
    pub fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, drawing_context: &mut DrawingContext) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::BLACK);
        let winning_label = graphics::Text::new("Winner :");
        graphics::draw(ctx, &winning_label, graphics::DrawParam::default().dest(Vec2::new(350. - winning_label.width(ctx), 350. - winning_label.height(ctx) / 2.)))?;
        self.winner_pawn.draw(drawing_context, ctx, Vec2::new(375., 350.), 2., false);
        Ok(())
    }
}