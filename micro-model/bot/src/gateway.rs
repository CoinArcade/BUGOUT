use micro_model_moves::{GameId, Player};
use serde_derive::{Deserialize, Serialize};

/// This command is sent from gateway, and
/// requests that robocall coordinate with
/// tinybrain to generate moves for a given
/// game ID and player.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttachBot {
    pub game_id: GameId,
    pub player: Player,
}

/// This reply is sent once a bot is listening
/// as a certain player in a certain game.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotAttached {
    pub game_id: GameId,
    pub player: Player,
}

impl AttachBot {
    pub fn from(bytes: &[u8]) -> Result<Self, std::boxed::Box<bincode::ErrorKind>> {
        bincode::deserialize(bytes)
    }
    pub fn serialize(&self) -> Result<Vec<u8>, std::boxed::Box<bincode::ErrorKind>> {
        Ok(bincode::serialize(&self)?)
    }
}
impl BotAttached {
    pub fn from(bytes: &[u8]) -> Result<Self, std::boxed::Box<bincode::ErrorKind>> {
        bincode::deserialize(bytes)
    }
    pub fn serialize(&self) -> Result<Vec<u8>, std::boxed::Box<bincode::ErrorKind>> {
        Ok(bincode::serialize(&self)?)
    }
}
