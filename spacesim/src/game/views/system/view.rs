use bevy::prelude::{
    default, shape, Added, Assets, Changed, Color, Commands, Entity, Image, Local, Mesh, PbrBundle,
    Query, RemovedComponents, Res, ResMut, Resource, StandardMaterial, Transform, Vec2, With,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::HashMap;
use spacesim_simulation::common::marker_components::{IsPlanet, IsShip};
use spacesim_simulation::ships::components::SystemCoordinates;

use super::SCALING_FACTOR;

type SimulationEntity = Entity;
type MirrorEntity = Entity;

#[derive(Resource, Default)]
pub(crate) struct MirrorMap {
    value: HashMap<SimulationEntity, MirrorEntity>,
}

pub(crate) fn mirror_planets(
    mut commands: Commands,
    q_new_planets: Query<(SimulationEntity, &SystemCoordinates), Added<IsPlanet>>,
    mut e_removed_planets: RemovedComponents<IsPlanet>,
    mut mirrored_entities: ResMut<MirrorMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    for (sim_entity, coords) in &q_new_planets {
        let mirrored_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 1_000. * 6371. / SCALING_FACTOR,
                        subdivisions: 50,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: debug_material.clone(),
                transform: Transform::from_translation(coords.value / SCALING_FACTOR),
                ..Default::default()
            })
            .id();
        mirrored_entities.value.insert(sim_entity, mirrored_entity);
    }

    for sim_entity in e_removed_planets.iter() {
        mirrored_entities.value.remove(&sim_entity);
    }
}

pub(crate) fn mirror_ships(
    mut commands: Commands,
    q_new_ships: Query<(SimulationEntity, &SystemCoordinates), Added<IsShip>>,
    mut e_removed_ships: RemovedComponents<IsShip>,
    mut mirrored_entities: ResMut<MirrorMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (sim_entity, coords) in &q_new_ships {
        let mirrored_entity = commands
            .spawn(PbrBundle {
                mesh: meshes.add(shape::Box::new(0.05, 0.05, 0.05).try_into().unwrap()),
                material: materials.add(StandardMaterial::from(Color::RED).into()),
                transform: Transform::from_translation(coords.value / SCALING_FACTOR),
                ..Default::default()
            })
            .id();
        mirrored_entities.value.insert(sim_entity, mirrored_entity);
    }

    for sim_entity in e_removed_ships.iter() {
        mirrored_entities.value.remove(&sim_entity);
    }
}

pub(crate) fn update_system_coordinates_based_transforms(
    q_simulation: Query<(SimulationEntity, &SystemCoordinates), Changed<SystemCoordinates>>,
    mut q_mirror: Query<&mut Transform>,
    mirrored_entities: Res<MirrorMap>,
) {
    for (sim_entity, coords) in &q_simulation {
        if let Some(mirror_entity) = mirrored_entities.value.get(&sim_entity) {
            if let Ok(mut transform) = q_mirror.get_mut(*mirror_entity) {
                *transform = Transform::from_translation(coords.value / SCALING_FACTOR);
            }
        }
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
