use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ColorPref {
    Black,
    White,
    Any,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionColorPref {
    pub session_id: SessionId,
    pub color_pref: ColorPref,
    pub client_id: ClientId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameColorPref {
    NotReady,
    Partial {
        game_id: GameId,
        pref: SessionColorPref,
    },
    Complete {
        game_id: GameId,
        prefs: (SessionColorPref, SessionColorPref),
    },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ClientId(pub Uuid);
#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct SessionId(pub Uuid);
#[derive(Clone, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub struct GameId(pub Uuid);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[cfg(test)]
impl GameId {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[cfg(test)]
impl ClientId {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[cfg(test)]
impl SessionId {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
