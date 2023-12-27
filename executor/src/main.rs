use libloading::{Library, library_filename, Symbol};
use std::io::Error;
use types::*;
use bytes::{Bytes};

fn main() {
    call_library();
}

fn call_library() {
    let data = vec![
        Event{event_name: "Auth", event_time: 123, data: Bytes::copy_from_slice(&[8, 136, 39])},
        Event{event_name: "Auth", event_time: 123, data: Bytes::copy_from_slice(&[8, 136, 39])},
        Event{event_name: "Payment", event_time: 123, data: Bytes::copy_from_slice(&[8, 196, 19, 26, 3, 87, 69, 66])},
        Event{event_name: "Auth", event_time: 123, data: Bytes::copy_from_slice(&[8, 136, 39])},
    ];

    unsafe {
        let lib = Library::new(library_filename("lib")).unwrap(); // Load the "hello_world" library

        let func: Symbol<unsafe extern fn(&Vec<Event>) -> Result<String, Error>> = lib.get(b"calculate").unwrap(); // Get the function pointer
        let result = func(&data);

        println!("{:?}", &result);
    }
}
