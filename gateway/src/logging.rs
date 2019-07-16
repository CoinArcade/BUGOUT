use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::model::Player;
use crate::websocket::WsSession;

pub fn emoji(player: &Player) -> String {
    match player {
        Player::BLACK => vec!["♚", "♛", "♜", "♝", "♞", "♟"]
            .choose(&mut rand::thread_rng())
            .map(|s| s.to_string())
            .unwrap_or("♚".to_owned()),
        Player::WHITE => vec!["♔", "♕", "♖", "♗", "♘", "♙"]
            .choose(&mut rand::thread_rng())
            .map(|s| s.to_string())
            .unwrap_or("♔".to_owned()),
    }
}

pub const EMPTY_SHORT_UUID: &str = "        ";
pub const MEGA_DEATH_STRING: &str = "⚰️⚰️⚰️⚰️⚰️⚰️⚰️⚰️";
pub fn short_uuid(uuid: Uuid) -> String {
    uuid.to_string()[..8].to_string()
}

pub fn session_code(ws_session: &WsSession) -> String {
    format!(
        "{} {}",
        short_uuid(ws_session.client_id),
        ws_session
            .current_game
            .map(|gid| short_uuid(gid))
            .unwrap_or(EMPTY_SHORT_UUID.to_string())
    )
    .to_string()
}
