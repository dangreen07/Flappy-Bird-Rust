use crate::resources::{Game, GameState, Player};
use bevy::prelude::*;
use bevy::render::mesh::MeshAabb;

pub fn collision_detection(
    mut game: ResMut<Game>,
    mesh_entities: Query<(&mut Mesh2d, &GlobalTransform)>,
    global_transforms: Query<&GlobalTransform>,
    meshes: Res<Assets<Mesh>>,
) {
    let player_transform = global_transforms.get(game.player.entity.unwrap()).unwrap();

    // Pipe collision detection
    let pipes = game.pipes.entities.clone();
    for pipe in pipes {
        let pipe_mesh = mesh_entities.get(pipe).unwrap();
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

    // Floor collision detection
    let floor = game.floor.unwrap();
    let floor_mesh = mesh_entities.get(floor).unwrap();
    let floor_id = floor_mesh.0.id();
    let mesh = meshes.get(floor_id).unwrap().clone();
    let collided = has_collided(&game.player, player_transform, mesh, floor_mesh.1);
    if collided {
        game.state = GameState::GameOver;
        return;
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
