use crate::model::Player;
pub fn other_player(player: Player) -> Player {
    match player {
        Player::BLACK => Player::WHITE,
        Player::WHITE => Player::BLACK,
    }
}
