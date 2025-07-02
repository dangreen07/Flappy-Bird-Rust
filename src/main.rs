use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, Wireframe2dPlugin::default()));

    app.add_systems(Startup, setup);

    app.add_systems(Update, sprite_movement);

    app.run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let mut sprite = Sprite::from_image(asset_server.load("FlappyBird.png"));

    sprite.custom_size = Some(Vec2::new(75.0, 75.0));

    commands.spawn((sprite, Transform::from_xyz(0., 0., 0.), Direction::Right));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
}
