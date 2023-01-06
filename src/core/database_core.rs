use redis::Client;

use crate::database::redis_proxy::RedisProxy;

pub struct database_core {
    client: RedisProxy,
}

impl database_core {
    pub fn new() -> database_core {
        database_core {
            client: RedisProxy::new(String::from("redis://127.0.0.1/")),
        }
    }
    pub fn find_collision(&self, url: String) -> bool {
        self.client.has(&url)
    }
    pub fn update_count(&self) {
        let current_count = self.get_count();
        let _res = self.client.set("count", &current_count.to_string());
    }

    pub fn get_count(&self) -> usize {
        self.client.get("count").unwrap().parse::<usize>().unwrap()
    }
}
