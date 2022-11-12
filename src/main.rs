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
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Octochess!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins);
    
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
    
    commands.spawn_bundle(camera)
    .insert_bundle(PickingCameraBundle::default());
}
