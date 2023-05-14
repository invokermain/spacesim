use std::f32::consts::PI;

use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, PointLight, PointLightBundle, Quat, ResMut,
    StandardMaterial, Transform, Vec3,
};

use bevy::utils::default;

use super::SystemViewHandles;

pub(super) fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut handles: ResMut<SystemViewHandles>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            range: 15.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

    let entity_id = commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.15,
                    subdivisions: 50,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        })
        .id();

    handles.as_mut().sun = entity_id;
}

pub(super) fn spawn_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut handles: ResMut<SystemViewHandles>,
) {
    let material = StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    };
    let m_handle = materials.add(material);

    let mesh = meshes.add(
        shape::Cylinder {
            radius: 0.005,
            height: 15.,
            resolution: 8,
            segments: 1,
        }
        .try_into()
        .unwrap(),
    );

    let axes_y_id = commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            material: m_handle.clone(),
            ..default()
        })
        .id();

    let axes_z_id = commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(Vec3::ZERO)
                .with_rotation(Quat::from_rotation_x(0.5 * PI)),
            material: m_handle.clone(),
            ..default()
        })
        .id();

    let axes_x_id = commands
        .spawn(PbrBundle {
            mesh,
            transform: Transform::from_translation(Vec3::ZERO)
                .with_rotation(Quat::from_rotation_z(0.5 * PI)),
            material: m_handle,
            ..default()
        })
        .id();

    handles.axes_lines = (axes_x_id, axes_y_id, axes_z_id);
}
