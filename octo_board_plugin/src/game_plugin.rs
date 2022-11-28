use bevy::{prelude::*,  
    sprite::MaterialMesh2dBundle, 
    render::mesh::*,
};

use bevy_mod_picking::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use crate::{
    resources::{
        tile_map::*,
        tile::*,
    },
    components::{
        tile_coord::*,
        shape::*,
        pawn::*,
    }
};

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

struct StartupParameters {
    octo_ratio: f32,
    tile_scale: f32,
    tile_gap: f32,
    border: f32,
}

fn setup_system(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let grid_size= 500_f32;
    let tile_scale = grid_size / QUAD_ON_SIDE as f32 * (2_f32.sqrt()/2_f32);

    let parameters = StartupParameters {
        octo_ratio: 0.25,
        tile_scale: tile_scale,
        tile_gap: tile_scale / 2.,
        border: 4.,
    };

    spawn_tiles(&mut commands, &mut meshes, &mut materials, &parameters);
    spawn_pawns(&mut commands, &mut meshes, &mut materials, &parameters);
}

fn spawn_tiles (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parameters: &StartupParameters) {
        let tile_map = TileMap::create(OCTO_ON_SIDE);
        for tile in tile_map.map {
            let (mesh, material, tile_transform, shape, coord) =  match tile {
                Tile::Quad(x, y) => {
                    (create_quad(parameters.octo_ratio, parameters.tile_scale, parameters.border).into(),
                    ColorMaterial::from(Color::RED),
                    Transform::default().with_translation(Vec3::new(x as f32 * parameters.tile_gap, y as f32 * parameters.tile_gap, 0_f32)),
                    Shape::Quad,
                    TileCoord{x: x as i32,y: y as i32})
                },
                Tile::Octo(x, y) => {
                    (create_octogone(parameters.octo_ratio, parameters.tile_scale, parameters.border).into(),
                    ColorMaterial::from(Color::GRAY),
                    Transform::default().with_translation(Vec3::new(x as f32 * parameters.tile_gap, y as f32 * parameters.tile_gap, 0_f32)),
                    Shape::Octo,
                    TileCoord{x: x as i32, y: y as i32})
                }
            };
            
            let bundle = MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(material),
                transform: tile_transform,
                ..default()};
    
            commands.spawn(bundle)
            .insert(shape)
            .insert(coord)
            .insert(PickableBundle::default())
            .insert(Name::new(format!("{} ({})", shape, coord)));
        }
}

fn spawn_pawns (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parameters: &StartupParameters)
{
    spawn_pawn(commands, meshes, materials, PlayerSide::Bottom, TileCoord {x: 3 , y: 1}, &parameters);
    spawn_pawn(commands, meshes, materials, PlayerSide::Bottom, TileCoord {x: 4 , y: 2}, &parameters);
    spawn_pawn(commands, meshes, materials, PlayerSide::Bottom, TileCoord {x: 5 , y: 1}, &parameters);
    
    spawn_pawn(commands, meshes, materials, PlayerSide::Top, TileCoord {x: 3 , y: 7}, &parameters);
    spawn_pawn(commands, meshes, materials, PlayerSide::Top, TileCoord {x: 4 , y: 6}, &parameters);
    spawn_pawn(commands, meshes, materials, PlayerSide::Top, TileCoord {x: 5 , y: 7}, &parameters);
}

fn spawn_pawn (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_side: PlayerSide,
    tile_coord: TileCoord,
    parameters : &StartupParameters)
{

    let label = match player_side {
        PlayerSide::Top => "Top",
        PlayerSide::Bottom => "Bottom",
    };

    let factor = parameters.tile_gap;
    let position = Vec3::new(tile_coord.x as f32 * factor, tile_coord.y as f32 * factor, 1_f32);

    commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1_f32, 32).into()).into(),
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(16.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..Default::default()
        })
        .insert(Pawn{player_side, position: Some(tile_coord)})
        .insert(Name::new(label));

}

pub fn input_system(mut events: EventReader<PickingEvent>, tiles: Query<(&TileCoord, &Shape)>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(_) => {},
            PickingEvent::Hover(_) => {},
            PickingEvent::Clicked(e) => {
                if let Ok((coord, shape)) = tiles.get(*e) {
                    info!("Click : {:?} - {}", shape , coord);
                }
            },
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system);
        
        app.add_plugin(PickingPlugin).add_plugin(InteractablePickingPlugin);
        app.add_system_to_stage(CoreStage::PostUpdate, input_system);
        
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<TileCoord>()
                .register_inspectable::<Shape>();
        }
    }
}