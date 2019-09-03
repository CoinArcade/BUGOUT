#![feature(bind_by_move_pattern_guards)]
#![feature(checked_duration_since)]
extern crate crossbeam;
extern crate crossbeam_channel;
extern crate dotenv;
extern crate envy;
extern crate harsh;
#[macro_use]
extern crate lazy_static;
extern crate mio_extras;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate ws;

mod client_commands;
mod client_events;
pub mod compact_ids;
pub mod env;
pub mod kafka_commands;
pub mod kafka_events;
pub mod kafka_io;
mod logging;
mod model;
pub mod router;
mod topics;
pub mod websocket;
