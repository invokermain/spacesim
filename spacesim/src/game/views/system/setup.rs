use bevy::prelude::{
    shape, Assets, Commands, Mesh, PbrBundle, PointLight, PointLightBundle, ResMut, Transform, Vec3,
};
use bevy::utils::default;

pub(super) fn spawn_sun(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 10.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Icosphere {
                radius: 0.15,
                subdivisions: 50,
            }
            .try_into()
            .unwrap(),
        ),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });
}
