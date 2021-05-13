use super::topics;
use crate::Components;
use log::error;
use redis::Client;
use redis_stream::consumer::{Consumer, ConsumerOpts, Message};
use std::collections::HashMap;

const BLOCK_MS: usize = 5000;
const GROUP_NAME: &str = "undo";
const CONSUMER_NAME: &str = "singleton";
const DATA_ID: &str = "data";

pub fn init(client: &Client, _components: Components) {
    let opts = || {
        ConsumerOpts::default()
            .group(GROUP_NAME, CONSUMER_NAME)
            .timeout(BLOCK_MS)
    };
    let mut unsorted: HashMap<String, StreamInput> = HashMap::new();

    let game_states_handler = |_id: &str, message: &Message| {
        for (key, v) in message.iter() {
            if key.trim().to_lowercase() == DATA_ID {
                if let redis::Value::Data(bytes) = v {
                    if let Some(stream_input) = bincode::deserialize(&bytes)
                        .map(|gs| StreamInput::LOG(gs))
                        .ok()
                    {
                        let k = key.clone();
                        unsorted.insert(k, stream_input.clone());
                    }
                }
            }
        }

        Ok(())
    };
    let bot_attached_handler = |_id: &str, _message: &Message| Ok(todo!());
    let undo_move_handler = |_id: &str, _message: &Message| Ok(todo!());

    let conn = || client.get_connection().expect("conn");
    let mut c1 = conn();
    let mut c2 = conn();
    let mut c3 = conn();
    let mut game_states_consumer = Consumer::init(
        &mut c1,
        topics::GAME_STATES_CHANGELOG,
        game_states_handler,
        opts(),
    )
    .expect("game states consumer init");
    let mut bot_attached_consumer =
        Consumer::init(&mut c2, topics::BOT_ATTACHED, bot_attached_handler, opts())
            .expect("game states consumer init");
    let mut undo_move_consumer =
        Consumer::init(&mut c3, topics::UNDO_MOVE, undo_move_handler, opts())
            .expect("undo move consumer init");

    /*
    let xrr = conn.xread_options(
            &[UNDO_MOVE, BOT_ATTACHED, GAME_STATES_CHANGELOG],
            &[READ_OP, READ_OP, READ_OP],
            opts,
        )?;
        let unsorted = deser(xrr)?;
        let mut sorted_keys: Vec<XReadEntryId> = unsorted.keys().map(|k| *k).collect();
        sorted_keys.sort();
        let mut answer = vec![];
        for sk in sorted_keys {
            if let Some(data) = unsorted.get(&sk) {
                answer.push((sk, data.clone()))
            }
        }
    */

    loop {
        if let Err(e) = game_states_consumer.consume() {
            error!("could not consume game states stream {:?}", e)
        }
        if let Err(e) = bot_attached_consumer.consume() {
            error!("could not consume BOT ATTACHED stream {:?}", e)
        }
        if let Err(e) = undo_move_consumer.consume() {
            error!("could not consume UNDO MOVE stream {:?}", e)
        }
    }
}

use super::undo::consume_undo;
use super::*;
use crate::repo::Botness;

fn consume_log(game_state: &GameState, reg: &Components) {
    if let Err(e) = reg.game_state_repo.put(&game_state) {
        error!("could not track game state: {:?}", e)
    }
}

fn consume_bot_attached(ba: &BotAttached, reg: &Components) {
    if let Err(e) = reg.botness_repo.put(&ba.game_id, ba.player, Botness::IsBot) {
        error!("could not track bot attached: {:?}", e)
    }
}
