use ggez::{*};
use glam::*;

mod shape_style;
mod tiles;
mod utils;
mod grid;
mod brain;
mod textures;
mod game;
use crate::game::*;

use std::{env, path};

fn main(){


    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    println!("{:?}", resource_dir);
    
    let mut c = conf::Conf::new();
    c.window_mode.width = 500_f32;
    c.window_mode.height = 500_f32;
    let (mut ctx, event_loop) = ContextBuilder::new("OctoChess", "AntonMakesGames").add_resource_path(resource_dir)
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
    
    let grid_position = Vec2::new(90., 90.);
    let game_instance = match Game::new(&mut ctx, grid_position) {Ok(game)=>game, Err(err)=> panic!("{0}", err)};

    event::run(ctx, event_loop, game_instance);
}
