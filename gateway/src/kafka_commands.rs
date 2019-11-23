use serde_derive::{Deserialize, Serialize};

use crate::model::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinPrivateGameKafkaCommand {
    #[serde(rename = "gameId")]
    pub game_id: GameId,
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindPublicGameKafkaCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChooseColorPrefKafkaCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "colorPref")]
    pub color_pref: ColorPref,
}

/// Gateway may manually create private games,
/// but it will never create a public game.
/// We omit specifying the game ID here, and
/// let game lobby choose it for us.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGameKafkaCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum HeartbeatType {
    WebSocketPong,
    UserInterfaceBeep,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientHeartbeat {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "heartbeatType")]
    pub heartbeat_type: HeartbeatType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KafkaCommands {
    MakeMove(MakeMoveCommand),
    ProvideHistory(ProvideHistoryCommand),
    JoinPrivateGame(JoinPrivateGameKafkaCommand),
    FindPublicGame(FindPublicGameKafkaCommand),
    CreateGame(CreateGameKafkaCommand),
    ChooseColorPref(ChooseColorPrefKafkaCommand),
    ClientHeartbeat(ClientHeartbeat),
}