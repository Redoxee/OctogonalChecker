use ggez::{
        GameResult,
        graphics,
    };

pub struct Sprite {
    pub sprites: Vec<graphics::Rect>,
    pub size: glam::Vec2,
}

pub struct CharacterSprite {
    pub sprite: Sprite,
    pub sprite_selected: Sprite,
}

pub struct GameTextures {
    pub spritesheet: ggez::graphics::Image,
    pub spear_sprites: CharacterSprite,
    pub knight_sprites: CharacterSprite,
}

fn extract_frame_from_data(sheet: &aseprite::SpritesheetData, index: usize) -> graphics::Rect {
    let frame = &sheet.frames[index].frame;
    graphics::Rect { 
        x: frame.x as f32 / sheet.meta.size.w as f32, 
        y: frame.y as f32 / sheet.meta.size.h as f32, 
        w: frame.w as f32 / sheet.meta.size.w as f32, 
        h: frame.h as f32 / sheet.meta.size.h as f32
    }
}

impl GameTextures {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<GameTextures>{
        let file = ggez::filesystem::open(ctx, "/goblins.json")?;
        let mut spritesheet = match graphics::Image::new(ctx, "/goblins.png") {GameResult::Ok(i)=> i, GameResult::Err(e) => panic!("{}", e)};
        spritesheet.set_filter(graphics::FilterMode::Nearest);
        let spritesheet_data: aseprite::SpritesheetData = serde_json::from_reader(file).unwrap();
 
        let spear = CharacterSprite {
            sprite: Sprite {
                sprites: vec![
                    extract_frame_from_data(&spritesheet_data, 0),
                    extract_frame_from_data(&spritesheet_data, 1),
                ],
                size: glam::Vec2::new(spritesheet_data.frames[0].frame.w as f32, spritesheet_data.frames[0].frame.h as f32),
            },
            sprite_selected: Sprite {
                sprites: vec![
                    extract_frame_from_data(&spritesheet_data, 2),
                    extract_frame_from_data(&spritesheet_data, 3),
                ],
                size: glam::Vec2::new(spritesheet_data.frames[2].frame.w as f32, spritesheet_data.frames[2].frame.h as f32),
            }
        };
        
        let knight = CharacterSprite {
            sprite: Sprite {
                sprites: vec![
                    extract_frame_from_data(&spritesheet_data, 4),
                    extract_frame_from_data(&spritesheet_data, 5),
                ],
                size: glam::Vec2::new(spritesheet_data.frames[4].frame.w as f32, spritesheet_data.frames[4].frame.h as f32),
            },
            sprite_selected: Sprite {
                sprites: vec![
                    extract_frame_from_data(&spritesheet_data, 6),
                    extract_frame_from_data(&spritesheet_data, 7),
                ],
                size: glam::Vec2::new(spritesheet_data.frames[5].frame.w as f32, spritesheet_data.frames[5].frame.h as f32),
            }
        };

        let result = GameTextures{
            spritesheet,
            spear_sprites: spear,
            knight_sprites: knight,
        };

        Ok(result)
    }
}