use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use spatial::{EntityToTrack, SpatialHash2d, SpatialHash2dPlugin};

use rand::Rng;
mod spatial;

/**
 * - Create 2d vec array each frame
 * - Loop over each tracked entity
 * - Calculate index from x,y coordinates
 * - Push entity at index
 */

const MAP_SIZE: f32 = 300.;
const MAP_CELL_SIZE: f32 = 5.;

#[derive(Component)]
struct Thing {
    direction: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpatialHash2dPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(SpatialHash2d::new(MAP_SIZE, MAP_CELL_SIZE))
        .add_startup_system(startup)
        .add_system(update)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(MAP_SIZE / 2., MAP_SIZE / 2., 999.),
        ..default()
    });

    for _ in 0..30 {
        let mut rng = rand::thread_rng();
        let direction = Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 0.) * 100.;
        commands.spawn((Transform::IDENTITY, EntityToTrack, Thing { direction }));
    }
}

fn update(mut transforms: Query<(&mut Transform, &Thing)>, time: Res<Time>) {
    for (mut transform, thing) in transforms.iter_mut() {
        transform.translation += thing.direction * time.delta_seconds();

        transform.translation.x = (transform.translation.x + MAP_SIZE) % MAP_SIZE;
        transform.translation.y = (transform.translation.y + MAP_SIZE) % MAP_SIZE;
    }
}
