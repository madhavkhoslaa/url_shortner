extern crate redis;
use redis::{Client, Commands, RedisError};
pub struct RedisProxy {
    client: Result<Client, RedisError>,
}
impl RedisProxy {
    pub fn new(url: String) -> RedisProxy {
        RedisProxy {
            client: redis::Client::open(&*url),
        }
    }
    pub fn has(&self, key: &str) -> bool {
        let result: bool = self.client.as_ref().unwrap().exists(key).unwrap();
        return result;
    }
    pub fn get(&self, key: &str) -> Result<String, ()> {
        match self.client.as_ref().unwrap().get(key) {
            Ok(result) => Ok(result),
            Err(_e) => Err(()),
        }
    }
    pub fn set(&self, key: &str, value: &str) -> Result<String, ()> {
        match self.client.as_ref().unwrap().set(key, value) {
            Ok(result) => Ok(result),
            Err(_e) => Err(()),
        }
    }
}
