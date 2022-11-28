use bevy::{
    prelude::*, 
    core_pipeline::clear_color::ClearColorConfig, 
};

use octo_board_plugin::game_plugin::GamePlugin;
use bevy_mod_picking::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{WorldInspectorPlugin};

fn main() {
    let mut app = App::new();
    let window_descriptor = WindowDescriptor {
        title: "Octochess!".to_string(),
        width: 700.,
        height: 800.,
        ..default()
    };

    // Bevy default plugins
    app.add_plugins(DefaultPlugins.set(WindowPlugin {window: window_descriptor, ..default()}));
    
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    {
        app.add_plugin(WorldInspectorPlugin::new());
    }
    // Startup system (cameras)
    app.add_startup_system(camera_setup);
    // Run the app

    app.add_plugin(GamePlugin{});
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color =  ClearColorConfig::Custom(Color::BLACK);
    
    commands.spawn(camera)
    .insert(PickingCameraBundle::default());
}
