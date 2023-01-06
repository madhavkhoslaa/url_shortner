extern crate redis;
use redis::{Client, Commands, RedisError};
use serde_json;

pub struct RedisProxy {
    client: Result<Client, RedisError>,
}
impl RedisProxy {
    pub fn new(url: String) -> RedisProxy {
        RedisProxy {
            client: redis::Client::open(&*url),
        }
    }
    pub fn get(&self, key: &str) -> Result<String, ()> {
        match self.client.as_ref().unwrap().get(key) {
            Ok(result) => Ok(result),
            Err(E) => Err(()),
        }
    }
    pub fn set(&self, key: &str, value: &str) -> Result<String, ()> {
        match self.client.as_ref().unwrap().set(key, value) {
            Ok(result) => Ok(result),
            Err(E) => Err(()),
        }
    }
}
