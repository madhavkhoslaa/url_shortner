use crate::database::redis_proxy::RedisProxy;
#[derive(Debug, Clone)]
pub struct DatabaseCore {
    pub client: RedisProxy,
}

impl DatabaseCore {
    pub fn new(url: String) -> DatabaseCore {
        DatabaseCore {
            client: RedisProxy::new(url),
        }
    }
    pub fn find_collision(&self, url: String) -> bool {
        self.client.has(&url)
    }
    pub fn update_count(&self) {
        let mut current_count = self.get_count();
        current_count = current_count + 1;
        let _res = self.client.set("count", &current_count.to_string());
    }

    pub fn get_count(&self) -> usize {
        match self.client.get("count") {
            Ok(count) => count.parse::<usize>().unwrap(),
            Err(E) => 0,
        }
    }
}
