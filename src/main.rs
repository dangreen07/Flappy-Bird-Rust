use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use flappy_bird_rust::collision::*;
use flappy_bird_rust::constants::*;
use flappy_bird_rust::resources::*;
use flappy_bird_rust::setup::*;
use rand::Rng;

fn main() {
    let mut app = App::new();

    let mut log_plugin = LogPlugin::default();

    if cfg!(debug_assertions) {
        log_plugin.level = Level::INFO;
    } else {
        log_plugin.level = Level::WARN;
    }

    app.add_plugins(DefaultPlugins.set(log_plugin));

    app.init_resource::<Game>();

    app.add_systems(Startup, (setup, setup_pipes, setup_floor));

    app.add_systems(
        Update,
        (
            sprite_movement,
            pipe_update,
            collision_detection,
            animate_frames,
        )
            .run_if(game_running),
    );

    app.run();
}

fn game_running(game: Res<Game>) -> bool {
    match game.state {
        GameState::Playing => true,
        GameState::GameOver => false,
    }
}

fn calculate_pipe_heights(height: f32, window_height: f32) -> (f32, f32) {
    // If height is 0 then top is window_height and bottom is -100
    // If height is 1 then top is 100 and bottom is -window_height
    let top = (1. - height) * (window_height - PIPE_GAP) + PIPE_GAP;
    let bottom = -PIPE_GAP - height * (window_height - PIPE_GAP);
    (top, bottom)
}

fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut acceleration = game.player.acceleration;
    if keyboard_input.just_released(KeyCode::Space) {
        acceleration.y = JUMP_FORCE;
    }

    game.player.velocity += acceleration * time.delta_secs();

    // Apply the velocity
    let velocity = game.player.velocity;
    let mut transform = transforms.get_mut(game.player.entity.unwrap()).unwrap();
    transform.translation += velocity * time.delta_secs();
    *transforms.get_mut(game.player.entity.unwrap()).unwrap() = *transform;
}

fn pipe_update(
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    mut game: ResMut<Game>,
    window: Query<&Window>,
    mut commands: Commands,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let size = window.size(); // (width, height)

    let mut max_width: f32 = 0.;

    game.pipes.entities.retain(|pipe| {
        let mut transform = *transforms.get_mut(*pipe).unwrap();
        let pos = transform.translation; // (width, height, z-axis) z-axis will always be zero for 2D
        if pos.x > max_width {
            max_width = pos.x;
        }
        if pos.x < -size.x / 2. {
            let mut commands = commands.entity(*pipe);
            commands.despawn();
            return false;
        }
        // Move the pipe across
        transform.translation.x -= PIPE_MOVEMENT_SPEED * time.delta_secs();
        *transforms.get_mut(*pipe).unwrap() = transform;

        return true;
    });

    if max_width < size.x / 2. - PIPE_OFFSET {
        // Create a new pipe
        let mut rng = rand::rng();
        let random_number: f32 = rng.random_range(0.2..=0.8);
        let (top_pos, bottom_pos) = calculate_pipe_heights(random_number, size.y);
        let mesh = game.pipes.mesh.clone().unwrap();
        let material = game.pipes.material.clone().unwrap();
        let top_entity = commands
            .spawn((
                mesh.clone(),
                material.clone(),
                Transform::from_xyz(size.x / 2., top_pos, 0.),
            ))
            .id();

        let bottom_entity = commands
            .spawn((
                mesh,
                material,
                Transform::from_xyz(size.x / 2., bottom_pos, 0.),
            ))
            .id();

        game.pipes.entities.push(top_entity);
        game.pipes.entities.push(bottom_entity);
    }
}

fn animate_frames(time: Res<Time>, mut query: Query<(&mut FrameAnimation, &mut Sprite)>) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current = (anim.current + 1) % anim.frames.len();
            sprite.image = anim.frames[anim.current].clone();
        }
    }
}
