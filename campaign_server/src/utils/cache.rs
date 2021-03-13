use crate::utils::config::CONFIG;
use crate::utils::errors::ApiError;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::web::{Data, ServiceConfig};
use redis_async::resp::{FromResp, RespValue};

pub type Cache = Data<Addr<RedisActor>>;

#[allow(dead_code)]
pub async fn get_redis_entry<'a>(redis: Cache, key: &'a str) -> Result<String, ApiError> {
    let command = resp_array!["GET", key];
    send(redis, command).await
}

/// Insert or update an entry in redis
#[allow(dead_code)]
pub async fn set<'a>(redis: Cache, key: &'a str, value: &'a str) -> Result<String, ApiError> {
    let command = resp_array!["SET", key, value];
    send(redis, command).await
}

/// Delete an entry in redis
#[allow(dead_code)]
pub async fn delete<'a>(redis: Cache, key: &'a str) -> Result<String, ApiError> {
    let command = resp_array!["DEL", key];
    send(redis, command).await
}

/// Send a command to the redis actor
async fn send<'a>(redis: Cache, command: RespValue) -> Result<String, ApiError> {
    let error_message = format!("Could not send {:?} command to Redis", command);
    let error = ApiError::CacheError(error_message.into());
    let response = redis.send(Command(command)).await.map_err(|_| error)?;
    match response {
        Ok(message) => Ok::<String, _>(FromResp::from_resp(message).unwrap_or("".into())),
        Err(message) => Err(ApiError::CacheError(format!("{:?}", message))),
    }
}

/// Add the redis actor to actix data_types if the URL is set
pub fn add_cache(cfg: &mut ServiceConfig) {
    if let Ok(var) = std::env::var("REDIS_URL") {
        println!("Redis URL: {}",&var);
        let cache = RedisActor::start(&var);
        cfg.data(cache);
    } else {
        println!("Redis URL env var not found")
    }
}
