use bevy::{prelude::*,  
    sprite::MaterialMesh2dBundle, 
    sprite::Mesh2dHandle,
};

use crate::components::tile_coord::*;
use crate::components::shape::*;

pub const GRID_SIDE: usize = 4;
pub const NUMBER_OF_TILES: usize = (GRID_SIDE * 2 + 1)  * GRID_SIDE + GRID_SIDE + 1;
pub const TILES_ON_SIDE: usize = GRID_SIDE * 2 + 1;
pub const TILES_ON_ROW: usize = TILES_ON_SIDE;
pub const TILES_ON_COL: usize = GRID_SIDE + 1;

pub struct GamePlugin {
}

fn spawn_tiles(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let tile_size = 25_f32;
    for index in 0..NUMBER_OF_TILES {
        let coord = get_coord_from_index(index);
        let shape = get_tile_shape_from_index(index);
        let bundle = match shape {
            Shape::Octo => {
                let mut transform = Transform::identity();
                transform.rotate_z((1_f32 / 16_f32) * std::f32::consts::PI * 2.);
                transform.translation = Vec3::new(coord.x as f32 * tile_size, (coord.y + 1) as f32 * tile_size * 2., 0.);
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(tile_size, 8).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GRAY)),
                    transform: transform,
                    ..default()}
                },
            Shape::Quad => {
                let mut transform = Transform::identity();
                transform.translation = Vec3::new(coord.x as f32 * tile_size, coord.y as f32 * tile_size * 2., 0.);

                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(tile_size, 4).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: transform,
                    ..default()}
            },
        };

        commands.spawn_bundle(bundle).insert(coord).insert(shape);
    }
}

fn test_move(windows: Res<Windows>, mut transforms: Query<(&mut Transform, &mut Mesh2dHandle)>) {
    
    let window = windows.get_primary().unwrap();
    let position = window.cursor_position();
    if let Some(pos) = position {
        let pos = Vec3::new(pos.x, pos.y, 0_f32);
        for (mut transform, _) in &mut transforms {
            let dir = pos - transform.translation;
            transform.translation += dir * 0.15_f32;
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
    }
}