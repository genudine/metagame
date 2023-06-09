use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref SERVICE_ID: String = env::var("SERVICE_ID").unwrap();
}

pub fn service_id() -> String {
    SERVICE_ID.clone()
}

pub fn platform(world_id: i32) -> String {
    match world_id {
        1000 => "ps2ps4us".to_string(),
        2000 => "ps2ps4eu".to_string(),
        _ => "ps2".to_string(),
    }
}
