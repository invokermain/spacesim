use bevy::prelude::{
    default, shape, Assets, Commands, Entity, Image, Local, Mesh, PbrBundle, Query, ResMut,
    StandardMaterial, Transform, With,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::{HashMap, HashSet};
use spacesim_simulation::common::marker_components::IsPlanet;
use spacesim_simulation::ships::components::SystemCoordinates;

use super::SCALING_FACTOR;

type SimulationEntity = Entity;
type MirrorEntity = Entity;

/// The system view is a scaled down view of the solar system.
/// The scaling factor is 100_000_000:1.
pub(crate) fn view_system(
    mut commands: Commands,
    q_simulation: Query<(SimulationEntity, &SystemCoordinates), With<IsPlanet>>,
    mut q_mirror: Query<&mut Transform>,
    mut mirrored_entities: Local<HashMap<SimulationEntity, MirrorEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let mut leftover_mirrored_entities = HashSet::from_iter(mirrored_entities.keys().map(|&k| k));

    for (sim_entity, coords) in &q_simulation {
        if !mirrored_entities.contains_key(&sim_entity) {
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
            mirrored_entities.insert(sim_entity, mirrored_entity);
        } else {
            leftover_mirrored_entities.remove(&sim_entity);

            // update transform
            let mut transform = q_mirror.get_mut(mirrored_entities[&sim_entity]).unwrap();
            *transform = Transform::from_translation(coords.value / SCALING_FACTOR);
        }
    }

    for leftover_entity in leftover_mirrored_entities {
        commands.entity(leftover_entity).despawn();
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
