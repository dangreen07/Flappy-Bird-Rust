use bevy::render::mesh::MeshAabb;
use bevy::{prelude::*, sprite::Wireframe2dPlugin};
use rand::Rng;

enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    velocity: Vec3,
    acceleration: Vec3,
    collision_box: Rect,
}

#[derive(Default)]
struct Pipes {
    entities: Vec<Entity>,
    material: Option<MeshMaterial2d<ColorMaterial>>,
    mesh: Option<Mesh2d>,
}

#[derive(Resource)]
struct Game {
    player: Player,
    pipes: Pipes,
    floor: Option<Entity>,
    state: GameState,
}

impl Default for Game {
    fn default() -> Game {
        Game {
            player: Player::default(),
            pipes: Pipes::default(),
            floor: None,
            state: GameState::Playing,
        }
    }
}

const PIPE_OFFSET: f32 = 300.;
const PIPE_MOVEMENT_SPEED: f32 = 50.;
const PIPE_WIDTH: f32 = 75.;
const PIPE_GAP: f32 = 150.;

const JUMP_FORCE: f32 = 5_000.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(Wireframe2dPlugin::default());

    app.init_resource::<Game>();

    app.add_systems(Startup, (setup, setup_pipes, setup_floor));

    app.add_systems(
        Update,
        (sprite_movement, pipe_update, collision_detection).run_if(game_running),
    );

    app.run();
}

fn game_running(game: Res<Game>) -> bool {
    match game.state {
        GameState::Playing => true,
        GameState::GameOver => false,
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    commands.spawn(Camera2d);

    let width = 75.;
    let height = 75.;

    let custom_size = Vec2::new(width, height);

    let transform = Transform::from_xyz(0., 0., 0.);

    let sprite = Sprite {
        custom_size: Some(custom_size),
        image: asset_server.load("Grumpy Flappy Bird\\frame-1.png"),
        ..Default::default()
    };

    let collision_width = 50.;
    let collision_height = 50.;

    let collision_box = Rect::new(
        -collision_width / 2.,
        -collision_height / 2.,
        collision_width / 2.,
        collision_height / 2.,
    );

    let entity = commands.spawn((sprite, transform)).id();

    game.player.entity = Some(entity);
    game.player.acceleration = Vec3::new(0., -20., 0.); // Set gravity acceleration here
    game.player.collision_box = collision_box;
}

fn setup_pipes(
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

fn setup_floor(
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
        let random_number: f32 = rng.random_range(0.0..=1.0);
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

fn collision_detection(
    mut game: ResMut<Game>,
    pipe_meshes: Query<(&mut Mesh2d, &GlobalTransform)>,
    global_transforms: Query<&GlobalTransform>,
    meshes: Res<Assets<Mesh>>,
) {
    let player_transform = global_transforms.get(game.player.entity.unwrap()).unwrap();

    // Pipe collision detection
    let pipes = game.pipes.entities.clone();
    for pipe in pipes {
        let pipe_mesh = pipe_meshes.get(pipe).unwrap();
        let pipe_id = pipe_mesh.0.id();
        let mesh = meshes.get(pipe_id).unwrap().clone();
        let collided = has_collided(&game.player, player_transform, mesh, pipe_mesh.1);
        if collided {
            // Collision detected
            game.state = GameState::GameOver;
            println!("Collision detected!");
            return;
        }
    }
}

fn has_collided(
    player: &Player,
    player_global_transform: &GlobalTransform,
    mesh: Mesh,
    mesh_global_transform: &GlobalTransform,
) -> bool {
    let local_aabb = mesh.compute_aabb().unwrap();

    let min = local_aabb.min();
    let max = local_aabb.max();
    let local_corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(min.x, max.y, max.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(max.x, max.y, max.z),
    ];

    let world_corners: Vec<Vec3> = local_corners
        .iter()
        .map(|corner| mesh_global_transform.compute_matrix() * corner.extend(1.0))
        .map(|v4| v4.truncate())
        .collect();

    let mut ws_min = world_corners[0];
    let mut ws_max = world_corners[0];
    for &pt in &world_corners[1..] {
        ws_min = ws_min.min(pt);
        ws_max = ws_max.max(pt);
    }

    let global_2d_pos = Vec2::new(
        player_global_transform.translation().x,
        player_global_transform.translation().y,
    );

    let player_collision_box = Rect::from_corners(
        player.collision_box.min + global_2d_pos,
        player.collision_box.max + global_2d_pos,
    );

    let rect = Rect::from_corners(ws_min.truncate(), ws_max.truncate());

    let intersection = rect.intersect(player_collision_box);
    if !intersection.is_empty() {
        return true;
    }
    return false;
}
