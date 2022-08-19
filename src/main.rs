use ggez::*;
use glam::*;

mod shape_style;
mod tiles;
mod utils;
mod grid;
mod brain;
mod game;
use crate::game::*;

fn main(){

    let grid_position = Vec2::new(90., 90.);
    let game_instance = Game::new(grid_position);

    let mut c = conf::Conf::new();
    c.window_mode.width = 500_f32;
    c.window_mode.height = 500_f32;
    let (ctx, event_loop) = ContextBuilder::new("OctoChess", "AntonMakesGames")
    .default_conf(c)
    .window_setup(conf::WindowSetup{
        title:String::from("Octogonal chess"),
        samples: conf::NumSamples::One,
        vsync: true,
        srgb: true,
        icon:"".to_owned(),
    })
    .build()
    .unwrap();

    event::run(ctx, event_loop, game_instance);
}
