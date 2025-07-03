use bevy::{prelude::*, sprite::Wireframe2dPlugin};
use rand::Rng;

#[derive(Resource)]
struct Player {
    entity: Option<Entity>,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            entity: None,
            velocity: Vec3::new(0., 0., 0.),
            acceleration: Vec3::new(0., -20., 0.), // Gravity is enabled by default for player
        }
    }
}

#[derive(Default, Resource)]
struct Pipes {
    entities: Vec<Entity>,
    material: Option<MeshMaterial2d<ColorMaterial>>,
    mesh: Option<Mesh2d>,
}

const PIPE_OFFSET: f32 = 300.;
const PIPE_MOVEMENT_SPEED: f32 = 30.;
const PIPE_WIDTH: f32 = 75.;
const PIPE_GAP: f32 = 150.;

const JUMP_FORCE: f32 = 5_000.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(Wireframe2dPlugin::default());

    app.init_resource::<Player>();

    app.init_resource::<Pipes>();

    app.add_systems(Startup, setup);

    app.add_systems(Update, (sprite_movement, pipe_update));

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
    mut pipes: ResMut<Pipes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2d);

    let mut sprite = Sprite::from_image(asset_server.load("Grumpy Flappy Bird\\frame-1.png"));

    sprite.custom_size = Some(Vec2::new(75.0, 75.0));

    let transform = Transform::from_xyz(0., 0., 0.);

    let entity = commands.spawn((sprite, transform)).id();

    player.entity = Some(entity);

    // Initialize pipes
    let Ok(window) = window.single() else {
        return;
    };

    let size = window.size();
    let height = size.y;

    let pipe_color = Color::srgba(0., 1., 0., 1.);

    let material = materials.add(pipe_color);
    let material = MeshMaterial2d(material);

    let pipe = meshes.add(Rectangle::new(PIPE_WIDTH, height));
    let mesh = Mesh2d(pipe);

    pipes.material = Some(material);
    pipes.mesh = Some(mesh);
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
    mut player: ResMut<Player>,
    mut transforms: Query<&mut Transform>,
) {
    let mut acceleration = player.acceleration;
    if keyboard_input.just_released(KeyCode::Space) {
        acceleration.y = JUMP_FORCE;
    }

    player.velocity += acceleration * time.delta_secs();

    // Apply the velocity
    let velocity = player.velocity;
    let mut transform = transforms.get_mut(player.entity.unwrap()).unwrap();
    transform.translation += velocity * time.delta_secs();
    *transforms.get_mut(player.entity.unwrap()).unwrap() = *transform;
}

fn pipe_update(
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    mut pipes: ResMut<Pipes>,
    window: Query<&Window>,
    mut commands: Commands,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let size = window.size(); // (width, height)

    let mut max_width: f32 = 0.;

    pipes.entities.retain(|pipe| {
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
        let mesh = pipes.mesh.clone().unwrap();
        let material = pipes.material.clone().unwrap();
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

        pipes.entities.push(top_entity);
        pipes.entities.push(bottom_entity);
    }
}
