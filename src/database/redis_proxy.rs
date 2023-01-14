extern crate redis;
use redis::{Client, Commands};
#[derive(Debug, Clone)]

pub struct RedisProxy {
    client: Option<Client>,
}
impl RedisProxy {
    pub fn new(url: String) -> RedisProxy {
        match redis::Client::open(&*url) {
            Ok(client) => RedisProxy {
                client: Some(client),
            },
            Err(_) => {
                panic!("Unable to connect to redis !")
            }
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
