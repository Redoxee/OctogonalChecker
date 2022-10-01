
extern crate good_web_game as ggez;

mod board;
mod pawn;
mod shape_style;
mod tiles;
mod utils;
mod grid;
mod brain;
mod textures;

mod game_states;
mod game;
mod ui;

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
    
    ggez::start(
        ggez::conf::Conf::default()
            .cache(Some(include_bytes!("resources.tar")))
            .physical_root_dir(Some(resource_dir))
            .window_width(750)
            .window_height(750)
            .window_title("Octogonal Chess".to_owned()),
            //.sample_count(16),
        |mut context| Box::new(Game::new(&mut context).unwrap()),
    );
}
