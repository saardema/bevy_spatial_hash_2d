use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Map::new(100.0, 4))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(update_debug_quads)
        .add_system(update_map)
        .run();
}

#[derive(Component, Debug)]
pub struct EntityToTrack {
    current_cell: Option<Cell>,
}

impl EntityToTrack {
    fn new() -> Self {
        Self { current_cell: None }
    }
}

#[derive(Resource, Debug)]
struct Map {
    cells: Vec<Cell>,
    size: f32,
    subdivisions: f32,
}

impl Map {
    fn new(size: f32, subdivisions: u32) -> Self {
        let subdivisions = subdivisions as f32;
        let mut cells = Vec::new();
        let mut y = size / subdivisions / 2.;

        while y <= size {
            let mut x = size / subdivisions / 2.;
            while x <= size {
                cells.push(Cell::new((x, y)));
                x += size / subdivisions;
            }
            y += size / subdivisions;
        }

        Self {
            cells,
            size,
            subdivisions,
        }
    }

    fn get_cell_index(&self, pos: (f32, f32)) -> Option<usize> {
        let x = pos.0 / self.size * self.subdivisions;
        let y = pos.1 / self.size * self.subdivisions;

        let index = x as usize + (y * self.subdivisions) as usize;

        if index + 1 > self.cells.len() {
            return None;
        }

        Some(index)
    }

    fn add_entity(&mut self, entity: Entity, pos: (f32, f32)) {
        if let Some(index) = self.get_cell_index(pos) {
            self.cells[index].add(entity);
        }
    }

    fn update(
        &mut self,
        entity: Entity,
        entity_to_track: &mut EntityToTrack,
        transform: &Transform,
    ) {
        if let Some(index) = self.get_cell_index((transform.translation.x, transform.translation.y))
        {
            let mut cell = self.cells[index];
            if entity_to_track.current_cell != Some(cell) {
                if let Some(mut current_cell) = entity_to_track.current_cell {
                    current_cell.remove(entity)
                }
                cell.add(entity);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cell {
    items: Vec<Entity>,
    position: (f32, f32),
}

impl Cell {
    fn new(position: (f32, f32)) -> Self {
        Self {
            items: Vec::new(),
            position,
        }
    }

    fn add(&mut self, entity: Entity) {
        self.items.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        let index = self.items.iter().position(|&r| r == entity).unwrap();
        self.items.remove(index);
    }
}

fn setup(mut commands: Commands, map: ResMut<Map>) {
    commands.spawn((EntityToTrack::new(), SpatialBundle::default()));

    for cell in &map.cells {
        if cell.position.0 == map.cells[0].position.0 {
            println!();
        }
        print!(
            "({}, {:<3}) {:<8}",
            cell.position.0,
            cell.position.1,
            cell.items.len()
        );
    }
    println!();

    // Scene
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(map.size / 2., map.size / 2., 999.)
            .with_scale(Vec3::new(0.2, 0.2, 1.)),
        ..default()
    });

    for cell in &map.cells {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(map.size / map.subdivisions * 0.98)),
                    ..default()
                },
                transform: Transform::from_xyz(cell.position.0, cell.position.1, 0.),
                ..default()
            })
            .insert(DebugQuad(cell));
    }
}

#[derive(Component)]
struct DebugQuad;

// fn update_debug_quads(mut quads: Query<(&mut Sprite, &DebugQuad)>) {
//     for (mut sprite, quad) in quads.iter_mut() {
//         let mut alpha = (quad.0.items.len() + 1) as f32 / 10.;
//         alpha = alpha.clamp(0., 1.);
//         sprite.color.set_a(alpha);
//     }
// }

fn update_map(mut query: Query<(Entity, &Transform, &mut EntityToTrack)>, mut map: ResMut<Map>) {
    for (entity, transform, mut entity_to_track) in query.iter_mut() {
        map.update(entity, &mut entity_to_track, transform);
    }
}
