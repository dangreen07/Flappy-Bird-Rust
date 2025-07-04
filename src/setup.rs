use bevy::prelude::*;

use crate::constants::PIPE_WIDTH;
use crate::resources::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    commands.spawn(Camera2d);

    let width = 75.;
    let height = 75.;

    let custom_size = Vec2::new(width, height);

    let transform = Transform::from_xyz(0., 0., 0.);

    let mut frames = Vec::new();
    for i in 1..=8 {
        let path = format!("Grumpy Flappy Bird\\frame-{i}.png");
        frames.push(asset_server.load(path));
    }

    let mut sprite = Sprite::from_image(frames[0].clone());

    sprite.custom_size = Some(custom_size);

    let animation = FrameAnimation {
        frames,
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        current: 0,
    };

    let collision_width = 50.;
    let collision_height = 50.;

    let collision_box = Rect::new(
        -collision_width / 2.,
        -collision_height / 2.,
        collision_width / 2.,
        collision_height / 2.,
    );

    let entity = commands.spawn((sprite, transform, animation)).id();

    game.player.entity = Some(entity);
    game.player.acceleration = Vec3::new(0., -20., 0.); // Set gravity acceleration here
    game.player.collision_box = collision_box;
}

pub fn setup_pipes(
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let size = window.size();
    let height = size.y;

    let pipe_color = Color::srgba(0., 1., 0., 1.);

    let material = materials.add(pipe_color);
    let material = MeshMaterial2d(material);

    let pipe = meshes.add(Rectangle::new(PIPE_WIDTH, height));
    let pipe = Mesh2d(pipe);

    game.pipes.material = Some(material);
    game.pipes.mesh = Some(pipe);
}

pub fn setup_floor(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    let size = window.size();
    let floor_color = Color::srgba(139. / 255., 69. / 255., 19. / 255., 1.);

    let material = materials.add(floor_color);
    let material = MeshMaterial2d(material);

    let floor = meshes.add(Rectangle::new(size.x, 50.));
    let floor = Mesh2d(floor);

    let floor_entity = commands
        .spawn((
            floor,
            material,
            Transform::from_xyz(0., -size.y / 2. + 25., 0.5),
        ))
        .id();

    game.floor = Some(floor_entity);
}
