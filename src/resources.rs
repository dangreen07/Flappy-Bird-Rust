use bevy::prelude::*;

pub enum GameState {
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub frames: Vec<Handle<Image>>,
    pub timer: Timer,
    pub current: usize,
}

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub collision_box: Rect,
}

#[derive(Default)]
pub struct Pipes {
    pub entities: Vec<Entity>,
    pub material: Option<MeshMaterial2d<ColorMaterial>>,
    pub mesh: Option<Mesh2d>,
}

#[derive(Resource)]
pub struct Game {
    pub player: Player,
    pub pipes: Pipes,
    pub floor: Option<Entity>,
    pub state: GameState,
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
