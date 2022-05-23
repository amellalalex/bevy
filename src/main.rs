use bevy::prelude::*;

// Settings
const PLAYER_MOVE_FORCE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Speed {
    vx: f32,
    vy: f32,
}

#[derive(Component)]
struct Physics {
    speed: Speed,
    mass: f32,
}

#[derive(Component)]
struct Player {
    physics: Physics,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("img/melvin.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Physics {
            speed: Speed { vx: 0., vy: 0. },
            mass: 1.,
        });
}

fn handle_physics(time: Res<Time>, mut query: Query<(&mut Physics, &mut Transform)>) {
    for (mut physics, mut transform) in query.iter_mut() {
        transform.translation.x += physics.speed.vx * time.delta_seconds();
        transform.translation.y += physics.speed.vy * time.delta_seconds();
    }
}

fn handle_input(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Physics>) {}
