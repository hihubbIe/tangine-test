pub use tangine::ecs;

// Player - query only the player entity
pub struct PlayerEntity(pub ecs::Entity);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PlayerStateEnum {
    Idle = 0,
    Flying = 1,
}

pub struct PlayerState {
    pub current: PlayerStateEnum,
    pub previous: PlayerStateEnum,
}

pub struct Bounded;
