pub use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    Playing
}


// Game constants
pub const TILE_SIZE: u32 = 16;
pub const GAME_SCALE: f32 = 8.0;
pub const PLAYER_SPEED: f32 = 300.0;