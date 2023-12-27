use bytes::{Bytes};

pub struct Event<'a> {
    pub event_name: &'a str,
    pub event_time: i32,
    pub data: Bytes
}