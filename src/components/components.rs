#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PlayerStateEnum {
    Idle = 0,
    Flying = 1,
}

pub struct PlayerState {
    pub current: PlayerStateEnum,
    pub previous: PlayerStateEnum,
}
