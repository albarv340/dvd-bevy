use bevy::prelude::*;

const SIZE: f32 = 200.;
const SPEED: f32 = 100.;

#[allow(dead_code)]
fn main() {
    init();
}

fn init() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                visible: false,
                ..default()
            }),
            ..default()
        }),)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Movement(f32, f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(SIZE, SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(100., 0., 0.),
            texture: asset_server.load("./cucumber.png"),
            ..default()
        },
        Movement(SPEED, SPEED),
    ));
}


/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, window: Query<&Window>, mut sprite_position: Query<(&mut Movement, &mut Transform, &mut Sprite)>) {
    for (mut movement, mut transform, mut sprite) in &mut sprite_position {
        transform.translation.x += movement.0 * time.delta_seconds();
        transform.translation.y += movement.1 * time.delta_seconds();

        let half_width = window.single().resolution.width() / 2.;
        let half_height = window.single().resolution.height() / 2.;
        let half_size = SIZE / 2.;
        let mut collision_count = 0;
        if transform.translation.y + half_size > half_height {
            movement.1 = -SPEED;
            collision_count += 1;
            sprite.color = Color::rgb(0.75, 0.25, 0.25);
        }
        if transform.translation.y - half_size < -half_height {
            movement.1 = SPEED;
            collision_count += 1;
            sprite.color = Color::rgb(0.75, 0.75, 0.25);
        }
        if transform.translation.x - half_size < -half_width {
            movement.0 = SPEED;
            collision_count += 1;
            sprite.color = Color::rgb(0.25, 0.75, 0.25);
        }
        if transform.translation.x + half_size > half_width {
            movement.0 = -SPEED;
            collision_count += 1;
            sprite.color = Color::rgb(0.25, 0.25, 0.75);
        }
        if collision_count == 2 {
            sprite.color = Color::rgb(1., 1., 1.);
        }
    }
}
