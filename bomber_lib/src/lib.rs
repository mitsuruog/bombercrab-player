/// Used by the wasm export/import machinery
#[cfg(not(target_family = "wasm"))]
pub use anyhow;
pub use bincode;
use bomber_macro::wasm_wrap;
pub use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
pub use wasmtime;
#[cfg(not(target_family = "wasm"))]
use wasmtime::AsContextMut;

/// Useful definitions about the game world and the player's surroundings.
pub mod world;
use world::{Direction, Enemy, Object, Tile, TileOffset};

#[wasm_wrap]
pub trait Player: Default {
    /// This method defines your character. Every turn, you receive a view of your surroundings and must
    /// come up with an action to perform. Stay alive, find the hill and stay on it as long as possible!
    fn act(
        &mut self,
        surroundings: Vec<(Tile, Option<Object>, Option<Enemy>, TileOffset)>,
    ) -> Action;
    /// Limit of 10 characters.
    fn name(&self) -> String;
    /// Limit of 20 characters.
    fn team_name() -> String;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Action {
    Move(Direction),
    StayStill,
    /// Place a bomb at your current location.
    DropBomb,
    /// Place a bomb at your current location while moving.
    DropBombAndMove(Direction),
}
