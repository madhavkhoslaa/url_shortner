use actix_web::http::header::HeaderMap;
use serde::{Deserialize, Serialize};
extern crate iplocate;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Analytics {
    pub country: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

impl Analytics {
    pub fn new(value: &HeaderMap, ip: String) -> Analytics {
        let ip = String::from(ip).split(":").collect::<Vec<&str>>()[0].parse().unwrap();
        let country: Option<String>;
        let result = iplocate::lookup(ip).unwrap();
        if let Some(ref country_api) = &result.geo_ip.country {
            country = Some(String::from(country_api));
        } else {
            country = None;
        }

        Analytics {
            country,
            ip: Some(ip.to_string()),
            user_agent: Some(String::from(
                value.get("user-agent").unwrap().to_str().unwrap(),
            )),
        }
    }
}
