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
        let (mesh, material, tile_transform, shape, coord) =  match tile {
            Tile::Quad(x, y) => {
                (create_quad(octo_ratio, tile_scale, gap).into(),
                ColorMaterial::from(Color::RED),
                Transform::default().with_translation(Vec3::new(x as f32 * tile_gap, y as f32 * tile_gap, 0_f32)),
                Shape::Quad,
                TileCoord{x: x as i32,y: y as i32})
            },
            Tile::Octo(x, y) => {
                (create_octogone(octo_ratio, tile_scale, gap).into(),
                ColorMaterial::from(Color::GRAY),
                Transform::default().with_translation(Vec3::new(x as f32 * tile_gap, y as f32 * tile_gap, 0_f32)),
                Shape::Octo,
                TileCoord{x: x as i32, y: y as i32})
            }
        };
        
        let bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(material),
            transform: tile_transform,
            ..default()};

        commands.spawn_bundle(bundle)
            .insert(shape)
            .insert(coord)
            .insert(Name::new(format!("{} ({})", shape, coord)));
    }
}

fn spawn_pawns (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Query<(&TileCoord)>)
{
    commands.spawn().insert(Pawn::Top(Some(TileCoord{x: 3, y: 0}))).insert(Name::new("Top"));
    commands.spawn().insert(Pawn::Top(Some(TileCoord{x: 4, y: 1}))).insert(Name::new("Top"));
    commands.spawn().insert(Pawn::Top(Some(TileCoord{x: 5, y: 0}))).insert(Name::new("Top"));

    spawn_pawn(commands, meshes, materials, Pawn::Top(Some(TileCoord {x: 3 , y: 1})), tiles);
}

fn spawn_pawn (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pawn: Pawn,
    tiles: Query<(&TileCoord)>)
{
    let (coord, label) = match pawn {
        Pawn::Top(Some(coord)) => (coord, "Top"),
        Pawn::Bottom(Some(coord)) => (coord, "Bottom"),
        _=> panic!()
    };

    commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1_f32, 19).into()).into(),
            transform: Transform::default().with_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::splat(32.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..Default::default()
        })
        .insert(pawn)
        .insert(Name::new(label));

}

#[derive(Default)]
struct SelectedTile {
    pub entity : Option<Entity>
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
        app.add_startup_system(spawn_tiles);
        
        app.add_plugin(PickingPlugin).add_plugin(InteractablePickingPlugin);
        app.add_system_to_stage(CoreStage::PostUpdate, input_system);
        
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<TileCoord>()
                .register_inspectable::<Shape>();
        }
    }
}