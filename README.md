# bevy_spatial_hash_2d

A Bevy plugin for spatial partitioning in a 2 dimensions. Can be used to find nearby entities with much greater efficiency.

```rust
use bevy::prelude::*;

use spatial::{EntityToTrack, SpatialHash2d, SpatialHash2dPlugin};

const MAP_SIZE: f32 = 900.;
const MAP_CELL_SIZE: f32 = 20.;


fn main() {
    App::new()
        .add_plugin(SpatialHash2dPlugin)
        .insert_resource(SpatialHash2d::new(MAP_SIZE, MAP_CELL_SIZE))
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Transform::IDENTITY, EntityToTrack));
}

```
