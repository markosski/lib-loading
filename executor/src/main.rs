use libloading::{Library, library_filename, Symbol};
use std::io::Error;
use std::io::ErrorKind;
use types::*;

fn main() {
    let data = vec![
        Event{event_name: "Auth", event_time: 123, data: vec![8, 136, 39]},
        Event{event_name: "Auth", event_time: 123, data: vec![8, 136, 39]},
        Event{event_name: "Payment", event_time: 123, data: vec![8, 196, 19, 26, 3, 87, 69, 66]},
        Event{event_name: "Auth", event_time: 123, data: vec![8, 136, 39]},
    ];

    let functions = vec!["calculate", "calculate"];

    unsafe {
        let lib = Library::new(library_filename("lib")).unwrap(); // Load the "hello_world" library
        let results = call_dynamic_functions(&lib, &functions, &data);

        for result in results {
            if let Ok(calculation) = result {
                let json_string = String::from_utf8_lossy(&calculation.data);
                println!("{:?}, {:?}", &calculation.calculation_name, json_string);
            }
        }
    }
}

pub fn call_dynamic_functions(lib: &Library, funs: &Vec<&str>, data: &Vec<Event>) -> Vec<Result<CalculationResponse, Error>> {
    let mut responses = vec![];
    unsafe {
        for fun in funs {
            let response = call_library(lib, fun, data);
            responses.push(response);
        }
    }
    responses
}

unsafe fn call_library(lib: &Library, func_name: &str, data: &Vec<Event>) -> Result<CalculationResponse, Error> {
    let func: Result<Symbol<unsafe extern fn(&Vec<Event>) -> Result<CalculationResponse, Error>>, libloading::Error> = lib.get(func_name.as_bytes());
    let result = match func {
        Ok(f) => f(&data),
        Err(err) => Err(Error::new(ErrorKind::Other,format!("error executing dynamic library: {}", err.to_string())))
    };

    result
}

