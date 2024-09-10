use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
            SpriteBundle {
                texture: asset_server.load("penguin.png"),
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                ..default()
            },
            Direction::Right,
    ));
    commands.spawn((
            SpriteBundle {
                texture: asset_server.load("tree.png"),
                transform: Transform::from_xyz(100.0, 50.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 200.)),
                    ..default()
                },
                ..default()
            },
    ));
}

fn keyboard_input_system(time: Res<Time>, keyboard_input: Res<ButtonInput<KeyCode>>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    let speed = 300.0;
    for (mut penguin, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::KeyA){
            transform.translation.x -= speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            transform.translation.x += speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyW){
            transform.translation.y += speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            transform.translation.y -= speed * time.delta_seconds();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}
