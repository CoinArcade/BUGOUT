pub mod attached_bots;
pub mod entry_ids;
pub mod redis_keys;

pub use attached_bots::*;
pub use entry_ids::*;
use redis_conn_pool::redis;
#[derive(Debug)]
pub enum RepoErr {
    Redis(redis::RedisError),
}
impl From<redis::RedisError> for RepoErr {
    fn from(r: redis::RedisError) -> Self {
        RepoErr::Redis(r)
    }
}
