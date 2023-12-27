use prost::Message;
use serde::{Deserialize, Serialize};
use types::*;
use std::io::Error;
use std::io::ErrorKind;

pub mod events {
    pub mod auth {
        include!(concat!(env!("OUT_DIR"), "/events.auth.rs"));
    }
}
use events::auth;

#[derive(Serialize, Deserialize)]
struct Aggregations {
    balance: i32,
    event_count: i32
}


#[no_mangle]
pub fn calculate(events: &Vec<Event>) -> Result<String, Error> {
    let mut total_balance: i32 = 0;
    let mut event_count: i32 = 0;
    for event in events {
        match event.event_name {
            "Payment" => {
                let payment = auth::Payment::decode(&*event.data).unwrap();
                total_balance -= payment.amount;
            },
            "Auth" => {
                let auth = auth::Auth::decode(&*event.data).unwrap();
                total_balance += auth.amount;
            },
            &_ => {
                panic!("event name {} not supported", event.event_name);
            }
        }
        event_count += 1;
    }

    let data = Aggregations{balance: total_balance, event_count: event_count};
    match serde_json::to_string(&data) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))

    }
}