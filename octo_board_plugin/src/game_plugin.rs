use std::collections::HashMap;

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
        maybe::*,
    }
};

pub const OCTO_ON_SIDE: usize = 4;
pub const QUAD_ON_SIDE: usize = OCTO_ON_SIDE + 1;
pub const TILES_ON_SIDE: usize = OCTO_ON_SIDE + QUAD_ON_SIDE;
pub const NUMBER_OF_TILES: usize = TILES_ON_SIDE * TILES_ON_SIDE;
pub const TILES_ON_ROW: usize = TILES_ON_SIDE;
pub const TILES_ON_COL: usize = TILES_ON_SIDE;

#[derive(Resource)]
struct GridVisualParameters {
    octo_ratio: f32,
    tile_scale: f32,
    tile_gap: f32,
    border: f32,
}

pub struct GamePlugin {
}

#[derive(Default, Resource)]
pub struct SelectedPawn {
    selected : Option<Entity>
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

fn setup_system(    
    mut commands: Commands,
    parameters: Res<GridVisualParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    let tiles = spawn_tiles(&mut commands, &mut meshes, &mut materials, &parameters);
    spawn_pawns(&mut commands, &parameters, &mut meshes, &mut materials, &tiles);
}

fn spawn_tiles (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parameters: &GridVisualParameters) -> HashMap<TileCoord, Entity> {
        let tile_map = TileMap::create(OCTO_ON_SIDE);
        let mut result = HashMap::new();
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
    
            let tile_id = commands.spawn(bundle)
            .insert(shape)
            .insert(coord)
            .insert(PickableBundle::default())
            .insert(Name::new(format!("{} ({})", shape, coord))).id();
            result.insert(coord, tile_id);
        }

    result
}


fn spawn_pawns(
    commands: &mut Commands,
    parameters: &Res<GridVisualParameters>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    tiles : &HashMap<TileCoord, Entity>) {
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Bottom, TileCoord {x: 3 , y: 1}, &parameters);
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Bottom, TileCoord {x: 4 , y: 2}, &parameters);
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Bottom, TileCoord {x: 5 , y: 1}, &parameters);
        
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Top, TileCoord {x: 3 , y: 7}, &parameters);
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Top, TileCoord {x: 4 , y: 6}, &parameters);
        spawn_pawn(commands, meshes, materials, &tiles, PlayerSide::Top, TileCoord {x: 5 , y: 7}, &parameters);
}

macro_rules! unwrap_option_or {
    ($e:expr, $or_do_what:expr) => {
        if let Some(d) = $e { d } else { $or_do_what }
    };
}

macro_rules! unwrap_result_or {
    ($e:expr, $or_do_what:expr) => {
        if let Ok(d) = $e { d } else { $or_do_what }
    };
}

fn spawn_pawn (
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    tiles : &HashMap<TileCoord, Entity>,
    player_side: PlayerSide,
    tile_coord: TileCoord,
    parameters : & GridVisualParameters) {

    let (label, color )= match player_side {
        PlayerSide::Top => ("Top", Color::BEIGE),
        PlayerSide::Bottom => ("Bottom", Color::BLUE),
    };

    let tile = tiles.get(&tile_coord);
    let tile = unwrap_option_or!(tile, panic!("Creating a pawn on a non existing tile {:?}, {}", tile_coord, tiles.len()));

    let factor = parameters.tile_gap;
    let position = Vec3::new(tile_coord.x as f32 * factor, tile_coord.y as f32 * factor, 1_f32);

    commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(1_f32, 32).into()).into(),
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(16.)),
            material: materials.add(ColorMaterial::from(color)),
            ..Default::default()
        })
        .insert(Pawn{player_side, coord: tile_coord})
        .insert(Name::new(label))
        .insert(MaybeEntity{entity: Some(*tile)});
}

pub fn input_system(
    mut events: EventReader<PickingEvent>, 
    mut selected_piece: ResMut<SelectedPawn>,
    mut tiles : Query<(Entity, &TileCoord, &Shape)>, 
    mut pawns : Query<(Entity, &mut Pawn)>,
    mut maybe_entities : Query<&mut MaybeEntity>,
    mut transforms : Query<&mut Transform>) {

    for event in events.iter() {
        match event {
            PickingEvent::Selection(_) => {},
            PickingEvent::Hover(_) => {},
            PickingEvent::Clicked(e) => {
                if let Ok((clicked_tile, tile_coord, shape)) = tiles.get_mut(*e) {
                    info!("Click : {:?} - {}", shape , tile_coord);
                    let mut pawn_holder = unwrap_result_or!(maybe_entities.get_mut(clicked_tile), panic!());
                    
                    let mut clicked_pawn = None;
                    if let Some(clicked_pawn_entity) = pawn_holder.entity {
                        if let Ok(clicked) = pawns.get_mut(clicked_pawn_entity)
                        {
                            clicked_pawn = Some(clicked.0);
                        } 
                    }

                    if let Some(pawn) = clicked_pawn {
                        selected_piece.selected = Some(pawn);
                    }
                    else
                    {
                        if let Some(selected_pawn) = selected_piece.selected {
                            let target_position = if let Ok(tile_transform) = transforms.get_mut(clicked_tile) {
                                Vec3::new(tile_transform.translation.x, tile_transform.translation.y, 1_f32)
                            }
                            else
                            {
                                panic!()
                            };
                            
                            if let Ok((pawn_entity, mut pawn)) = pawns.get_mut(selected_pawn) {
                                if let Ok(mut pawn_transform) = transforms.get_mut(selected_pawn) {
                                    pawn_transform.translation = target_position;
                                    pawn.coord = *tile_coord;
                                }
                                
                                pawn_holder.entity = Some(pawn_entity);

                                let target_entity = maybe_entities.get_mut(pawn_entity);
                                let mut target_entity = unwrap_result_or!(target_entity, panic!());
                                target_entity.entity = Some(clicked_tile);
                            }

                            let source_tile = maybe_entities.get_mut(selected_pawn);
                            let mut source_tile = unwrap_result_or!(source_tile, panic!());
                            source_tile.entity = None;
                        }

                        selected_piece.selected = None;
                    }
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
        app.insert_resource(SelectedPawn{selected: None});

        let grid_size= 500_f32;
        let tile_scale = grid_size / QUAD_ON_SIDE as f32 * (2_f32.sqrt()/2_f32);
        let parameters = GridVisualParameters {
            octo_ratio: 0.25,
            tile_scale: tile_scale,
            tile_gap: tile_scale / 2.,
            border: 4.,
        };

        app.insert_resource(parameters);

        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<TileCoord>()
                .register_inspectable::<Shape>();
        }
    }
}