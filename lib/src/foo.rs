
pub mod events {
    pub mod auth {
        include!(concat!(env!("OUT_DIR"), "/events.auth.rs"));
    }
}
use events::auth;
use prost::Message;

fn main() {
    let mut payment = auth::Payment::default();
    payment.amount = 2500;
    payment.channel = "WEB".to_string();
    let bytes = payment.encode_to_vec();
    println!("{:?}", &bytes);

    let mut auth = auth::Auth::default();
    auth.amount = 5000;
    let bytes2 = auth.encode_to_vec();
    println!("{:?}", &bytes2);
}