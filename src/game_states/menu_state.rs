use ggez::{
    *,
};

use crate::game::*;
use crate::ui::*;

pub struct MenuState {
    one_player: Button,
    two_player: Button,
}

pub enum MenuOption {
    None,
    OnePlayer,
    TwoPlayer,
}

impl MenuState {
    pub fn new() -> MenuState{
        MenuState {
            one_player: Button::new("1 Player", graphics::Rect::new_i32(200, 450, 150, 60)),
            two_player: Button::new("2 Players", graphics::Rect::new_i32(380, 450, 150, 60)),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, _drawing_context: &mut DrawingContext) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::BLACK);
        self.one_player.draw(ctx)?;
        self.two_player.draw(ctx)?;
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