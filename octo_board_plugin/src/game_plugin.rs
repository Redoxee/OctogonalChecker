use bevy::{prelude::*,  
    sprite::MaterialMesh2dBundle, 
    sprite::Mesh2dHandle,
    render::mesh::*,
    render::camera::RenderTarget,
};

use bevy_inspector_egui::RegisterInspectable;

use crate::resources::tile_map::*;
use crate::resources::tile::*;
use crate::components::tile_coord::*;
use crate::components::shape::*;

pub const OCTO_ON_SIDE: usize = 4;
pub const QUAD_ON_SIDE: usize = OCTO_ON_SIDE + 1;
pub const GRID_SIDE: usize = 4;
pub const NUMBER_OF_TILES: usize = (GRID_SIDE * 2 + 1)  * GRID_SIDE + GRID_SIDE + 1;
pub const TILES_ON_SIDE: usize = GRID_SIDE * 2 + 1;
pub const TILES_ON_ROW: usize = TILES_ON_SIDE;
pub const TILES_ON_COL: usize = GRID_SIDE + 1;

pub struct GamePlugin {
}

fn create_quad(octogon_ratio: f32, size: f32, thickness: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let s = size * (1. - octogon_ratio) / 2.0 - thickness / 2.0;
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        [s, 0.0, 0.0],
        [0.0, s, 0.0],
        [-s, 0.0, 0.0],
        [0.0, -s, 0.0],
    ]);

    mesh.set_indices(Some(Indices::U32(vec![
        0,1,2,
        0,2,3,])));

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, -1.0]; 4]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        [1.0, 0.5],
        [0.5, 1.0],
        [0.0, 0.5],
        [0.5, 0.0],
    ]);
    mesh
}

fn create_octogone(octogon_ratio: f32, size: f32, thickness: f32) -> Mesh {
    let size = size / 2.0;
    let inner_size = size - thickness / 2.;
    let inner_half = octogon_ratio * (size - thickness/2.);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        [inner_size, inner_half, 0.0], 
        [inner_half, inner_size, 0.0], 
        [-inner_half, inner_size, 0.0], 
        [-inner_size, inner_half, 0.0], 
        [-inner_size, -inner_half, 0.0], 
        [-inner_half, -inner_size, 0.0], 
        [inner_half, -inner_size, 0.0], 
        [inner_size, -inner_half, 0.0]]);

    mesh.set_indices(Some(Indices::U32(vec![
        0,1,2,
        0,2,3,
        0,3,4,
        0,4,5,
        0,5,6,
        0,6,7])));

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, -1.0]; 8]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
    ]);
    mesh
}

fn spawn_tiles(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let grid_size= 500_f32;
    let tile_scale = grid_size / QUAD_ON_SIDE as f32 * (2_f32.sqrt()/2_f32);

    let octo_ratio = 0.25;
    let gap = 4.;

    let tile_gap = tile_scale / 2.;
    let tile_map = TileMap::create(OCTO_ON_SIDE);
    for tile in tile_map.map {
        match tile {
            Tile::Quad(x, y) => {
                let bundle = MaterialMesh2dBundle {
                    mesh: meshes.add(create_quad(octo_ratio, tile_scale, gap).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_xyz(x as f32 * tile_gap, y as f32* tile_gap, 0.0),
                    ..default()};
    
                let coord = TileCoord{x : x as i32 , y: y as i32};
                commands.spawn_bundle(bundle)
                    .insert(coord)
                    .insert(Shape::Quad)
                    .insert(Name::new(format!("Quad ({})", coord)));
            },

            Tile::Octo(x, y) => {
                let bundle = MaterialMesh2dBundle {
                    mesh: meshes.add(create_octogone(octo_ratio, tile_scale, gap).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GRAY)),
                    transform: Transform::from_xyz(x as f32 * tile_gap, y as f32 * tile_gap, 0.0),
                    ..default()};
    
                let coord = TileCoord{x : x as i32, y: y as i32};
                commands.spawn_bundle(bundle)
                    .insert(coord)
                    .insert(Shape::Octo)
                    .insert(Name::new(format!("Octo ({})", coord)));
            }
        }
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


fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        eprintln!("World coords: {}", world_pos);
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
        app.add_system(my_cursor_system);
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<TileCoord>()
                .register_inspectable::<Shape>();
        }
    }
}