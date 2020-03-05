extern crate micro_changelog;

use micro_changelog::micro_model_moves::*;
use micro_changelog::redis_conn_pool;
use micro_changelog::redis_conn_pool::*;
use micro_changelog::repo::redis_key::*;
use micro_changelog::stream::*;
use micro_changelog::*;
use redis::Commands;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
const GAME_STATES_TOPIC: &str = "bugtest-game-states";
const MOVE_ACCEPTED_EV_TOPIC: &str = "bugtest-move-accepted-ev";
const MOVE_MADE_EV_TOPIC: &str = "bugtest-move-made-ev";
const GAME_READY_EV_TOPIC: &str = "bugtest-game-ready-ev";

#[test]
fn test_process_move() {
    let keys_to_clean = vec![];
    let streams_to_clean = vec![
        GAME_STATES_TOPIC,
        MOVE_ACCEPTED_EV_TOPIC,
        MOVE_MADE_EV_TOPIC,
        GAME_READY_EV_TOPIC,
    ];
    let pool = test_pool();
    panic_cleanup(
        streams_to_clean
            .clone()
            .iter()
            .map(|s| s.to_string())
            .collect(),
        keys_to_clean.clone(),
        pool.clone(),
    );

    thread::spawn(move || stream::process(test_topics(), &test_components(&test_pool())));
    thread::sleep(Duration::from_millis(100));

    let game_id = GameId(uuid::Uuid::new_v4());
    let mut conn = pool.get().unwrap();
    // Changelog expects a game ready event and will initialize
    // an empty game
    redis::cmd("XADD")
        .arg(GAME_READY_EV_TOPIC)
        .arg("MAXLEN")
        .arg("~")
        .arg("1000")
        .arg("*")
        .arg("game_id")
        .arg(game_id.0.to_string())
        .query::<String>(&mut *conn)
        .unwrap();

    let placement = Coord::of(0, 0);
    let move_made = MoveMade {
        game_id: game_id.clone(),
        reply_to: ReqId(Uuid::nil()),
        captured: vec![],
        coord: Some(placement),
        event_id: EventId::new(),
        player: Player::BLACK,
    };
    // Judge accepts a move
    redis::cmd("XADD")
        .arg(MOVE_ACCEPTED_EV_TOPIC)
        .arg("MAXLEN")
        .arg("~")
        .arg("1000")
        .arg("*")
        .arg("game_id")
        .arg(game_id.0.to_string())
        .arg("data")
        .arg(move_made.serialize().unwrap())
        .query::<String>(&mut *conn)
        .unwrap();
    // We should see something published to MOVE_MADE
    let xread_move_made = redis::cmd("XREAD")
        .arg("BLOCK")
        .arg(333)
        .arg("STREAMS")
        .arg(MOVE_MADE_EV_TOPIC)
        .arg("0-0")
        .query::<redis::Value>(&mut *conn)
        .unwrap();

    assert_ne!(xread_move_made, redis::Value::Nil);

    let xread_game_states_changelog = redis::cmd("XREAD")
        .arg("BLOCK")
        .arg(333)
        .arg("STREAMS")
        .arg(GAME_STATES_TOPIC)
        .arg("0-0")
        .query::<Vec<HashMap<String, Vec<HashMap<String, (String, String, String, Option<Vec<u8>>)>>>>>(&mut *conn)
        .unwrap();
    assert_eq!(xread_game_states_changelog.len(), 1);
    let by_timestamp = xread_game_states_changelog[0].get(GAME_STATES_TOPIC);
    assert!(by_timestamp.is_some());
    println!("by timestamp {:#?}", by_timestamp);
    let game_state_payload_vec: Vec<(String, String, String, Option<Vec<u8>>)> =
        by_timestamp.unwrap()[0].values().cloned().collect();
    let payload = &game_state_payload_vec[0];
    assert_eq!(payload.0, "game_id");
    assert_eq!(payload.1, game_id.0.to_string());
    assert_eq!(payload.2, "data");

    let expected_game_state = GameState {
        board: Board {
            pieces: [(placement, Player::BLACK)].iter().cloned().collect(),
            size: 19,
        },
        moves: vec![move_made],
        turn: 2,
        player_up: Player::WHITE,
        captures: Captures { black: 0, white: 0 },
    };
    assert_eq!(
        bincode::deserialize::<GameState>(&payload.3.as_ref().unwrap()).unwrap(),
        expected_game_state
    );

    clean_streams(
        streams_to_clean.iter().map(|s| s.to_string()).collect(),
        &pool,
    );
    clean_keys(keys_to_clean, &pool);
}

fn panic_cleanup(stream_names: Vec<String>, keys: Vec<String>, pool: Pool) {
    std::panic::set_hook(Box::new(move |e| {
        println!("{:#?}", e);
        clean_streams(stream_names.clone(), &pool);
        clean_keys(keys.clone(), &pool);
    }));
}
fn test_topics() -> StreamTopics {
    StreamTopics {
        game_states_changelog: GAME_STATES_TOPIC.to_string(),
        game_ready_ev: GAME_READY_EV_TOPIC.to_string(),
        move_accepted_ev: MOVE_ACCEPTED_EV_TOPIC.to_string(),
        move_made_ev: MOVE_MADE_EV_TOPIC.to_string(),
    }
}
fn test_pool() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
    redis_conn_pool::create(RedisHostUrl("redis://localhost".to_string()))
}
fn test_namespace() -> RedisKeyNamespace {
    RedisKeyNamespace("BUGTEST".to_string())
}
fn test_components(pool: &Pool) -> Components {
    Components {
        pool: pool.clone(),
        redis_key_provider: KeyProvider(test_namespace()),
    }
}

fn clean_keys(keys: Vec<String>, pool: &Pool) {
    let mut conn = pool.get().unwrap();
    for k in keys {
        conn.del(k.clone()).unwrap()
    }
}

fn clean_streams(stream_names: Vec<String>, pool: &Pool) {
    let mut conn = pool.get().unwrap();
    for sn in stream_names {
        match redis::cmd("XTRIM")
            .arg(&sn)
            .arg("MAXLEN")
            .arg("0")
            .query::<u32>(&mut *conn)
        {
            Err(e) => println!("Error in cleanup {}", e),
            Ok(count) => println!("Cleaned {} in {}", count, sn),
        }
    }
}