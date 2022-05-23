use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(handle_input)
        .add_system(handle_collisions)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Speed {
    vx: f32,
    vy: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("img/melvin.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Up)
        .insert(Speed { vx: 150., vy: 150. });
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform, &mut Speed)>,
) {
    for (mut logo, mut transform, mut speed) in sprite_position.iter_mut() {
        match *logo {
            Direction::Up => transform.translation.y += speed.vy * time.delta_seconds(),
            Direction::Down => transform.translation.y -= speed.vy * time.delta_seconds(),
            Direction::Left => transform.translation.x -= speed.vx * time.delta_seconds(),
            Direction::Right => transform.translation.x += speed.vx * time.delta_seconds(),
        }
    }
}

fn handle_input(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Direction>) {
    let mut direction = query.single_mut();

    if keyboard.pressed(KeyCode::Left) {
        *direction = Direction::Left;
    } else if keyboard.pressed(KeyCode::Right) {
        *direction = Direction::Right;
    } else if keyboard.pressed(KeyCode::Up) {
        *direction = Direction::Up;
    } else if keyboard.pressed(KeyCode::Down) {
        *direction = Direction::Down;
    }
}

fn handle_collisions(mut query: Query<(&mut Direction, &mut Transform)>) {
    for (mut direction, mut transform) in query.iter_mut() {
        if transform.translation.y <= 0. {
            // Flip
            *direction = Direction::Up;
        }
    }
}
