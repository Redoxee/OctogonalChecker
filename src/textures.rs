use ggez::{
        GameResult,
        graphics,
    };


pub struct GameTextures {
    pub spritesheet: ggez::graphics::Image,
    pub spear_sprites: Vec<graphics::Rect>,
    pub knight_sprites: Vec<graphics::Rect>,
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
 
        let spear_sprites = vec![
            extract_frame_from_data(&spritesheet_data, 0),
            extract_frame_from_data(&spritesheet_data, 1),
        ];

        let knight_sprites = vec![
            extract_frame_from_data(&spritesheet_data, 2),
            extract_frame_from_data(&spritesheet_data, 3),
        ];

        let result = GameTextures{
            spritesheet,
            spear_sprites,
            knight_sprites,
        };

        Ok(result)
    }
}