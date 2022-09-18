use ggez::{*,
    graphics,
};

use glam::Vec2;

use crate::game::{
    *,
    DrawingContext,
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Pawn {
    pub player : PlayerSide,
    pub table_index: usize,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct PawnArray {
    pub tile_indexes: [usize; MAX_PAWN_NUMBER],
    pub count: usize,
}

impl PawnArray {
    pub fn new() -> PawnArray {
        PawnArray {
            tile_indexes: [0; MAX_PAWN_NUMBER],
            count: 0,
        }
    }
}

impl Pawn {
    pub fn draw(&self, drawing_context: &mut DrawingContext, ctx:&mut ggez::Context, position: Vec2, scale: f32, is_selected: bool){
        let character = match self.player {PlayerSide::Bottom => &drawing_context.game_textures.spear_sprites, PlayerSide::Top => &drawing_context.game_textures.knight_sprites}; 
        let sprites = match is_selected { true => &character.sprite_selected, false => &character.sprite};
        let textures = &sprites.sprites;
        let frame_index = drawing_context.time as usize % textures.len();

        let mut param = graphics::DrawParam {
            src: textures[frame_index],
            ..Default::default()
        };

        let hs = sprites.size;
        param.trans = graphics::Transform::Values {
            dest: mint::Point2 {
                x: position.x - hs.x,
                y: position.y - hs.y,
            },
            offset: mint::Point2 {
                x: 0_f32,
                y: 0_f32,
            },
            rotation: 0_f32,
            scale: mint::Vector2 {x: scale, y: scale},
        };

        match graphics::draw(ctx, &drawing_context.game_textures.spritesheet, param)
        {
            GameResult::Err(e) => panic!("{}",e),
            GameResult::Ok(_) => (),
        };
    }
}