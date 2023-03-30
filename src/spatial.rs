use bevy::prelude::*;
use std::ops::Range;

pub struct SpatialHash2dPlugin;

impl Plugin for SpatialHash2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rebuild_map.in_schedule(CoreSchedule::Startup))
            .add_system(rebuild_map.in_base_set(CoreSet::PreUpdate))
            .add_startup_system(prepare_visualize_map)
            .add_system(visualize_map);
    }
}

#[derive(Component)]
pub struct EntityToTrack;

#[derive(Resource)]
pub struct SpatialHash2d {
    range: Range<f32>,
    total_cells: usize,
    cell_size: f32,
    columns: usize,
    cells: Vec<Vec<Entity>>,
}

impl SpatialHash2d {
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

fn rebuild_map(
    mut map: ResMut<SpatialHash2d>,
    entities: Query<(Entity, &Transform), With<EntityToTrack>>,
) {
    map.build();

    for (entity, transform) in &entities {
        map.insert_entity(entity, &transform.translation);
    }
}

fn print_map(map: Res<SpatialHash2d>) {
    for (i, cell) in map.cells.iter().enumerate() {
        if i % map.columns == 0 {
            println!();
        }
        print!("{} ", cell.len());
    }

    println!();
}

fn prepare_visualize_map(mut commands: Commands, map: Res<SpatialHash2d>) {
    for (i, _) in map.cells.iter().enumerate() {
        let x = i % map.columns;
        let y = i / map.columns;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(map.cell_size * 0.98)),
                ..default()
            },
            transform: Transform::from_xyz(x as f32 * map.cell_size, y as f32 * map.cell_size, 0.),
            ..default()
        });
    }
}

fn visualize_map(mut tiles: Query<&mut Sprite>, map: Res<SpatialHash2d>) {
    for (i, mut sprite) in tiles.iter_mut().enumerate() {
        let cell = &map.cells[i];
        let mut alpha = (cell.len() + 1) as f32 / 10.;
        alpha = alpha.clamp(0., 1.);
        sprite.color.set_a(alpha);
    }
}
