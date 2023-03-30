use std::ops::Range;

use bevy::{prelude::*, transform};

/**
 * - Create 2d vec array each frame
 * - Loop over each tracked entity
 * - Calculate index from x,y coordinates
 * - Push entity at index
 */

fn main() {
    App::new()
        .insert_resource(Map::new(100., 25.))
        .add_startup_system(startup)
        .add_system(rebuild_map.in_base_set(CoreSet::PreUpdate))
        .add_system(update)
        .add_plugins(DefaultPlugins)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((Transform::IDENTITY, EntityToTrack));
}

fn rebuild_map(mut map: ResMut<Map>, entities: Query<(Entity, &Transform), With<EntityToTrack>>) {
    map.build();

    for (entity, transform) in &entities {
        map.insert_entity(entity, &transform.translation);
    }
}

fn update(map: Res<Map>, mut transforms: Query<&mut Transform, With<EntityToTrack>>) {
    for mut transform in transforms.iter_mut() {
        transform.translation.x += 1.;
        transform.translation.y += 3.4;
        transform.translation.x = transform.translation.x % 100.;
        transform.translation.y = transform.translation.y % 100.;
    }

    for (i, cell) in map.cells.iter().enumerate() {
        if i % map.columns == 0 {
            println!();
        }
        print!("{} ", cell.len());
    }

    println!();
}

#[derive(Component)]
struct EntityToTrack;

#[derive(Resource)]
pub struct Map {
    range: Range<f32>,
    total_cells: usize,
    cell_size: f32,
    columns: usize,
    cells: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(size: f32, cell_size: f32) -> Self {
        let cells = vec![];
        let range = 0.0..size;
        let columns = (size / cell_size).ceil() as usize;
        let total_cells = columns * columns;

        Self {
            range,
            cell_size,
            total_cells,
            columns,
            cells,
        }
    }

    pub fn build(&mut self) {
        self.cells = vec![];

        for _ in 0..self.total_cells {
            self.cells.push(vec![]);
        }
    }

    pub fn insert_entity(&mut self, entity: Entity, position: &Vec3) {
        if let Some(index) = self.get_index(position) {
            self.cells[index].push(entity);
        }
    }

    fn get_index(&self, position: &Vec3) -> Option<usize> {
        if !self.range.contains(&position.x) || !self.range.contains(&position.y) {
            return None;
        }

        let index = (position.x / self.cell_size) as usize
            + (position.y / self.cell_size) as usize * self.columns;

        Some(index)
    }
}
