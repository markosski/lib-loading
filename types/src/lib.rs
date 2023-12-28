
#[derive(Debug)]
pub struct Event<'a> {
    pub event_name: &'a str,
    pub event_time: i32,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct CalculationResponse {
    pub calculation_name: String,
    pub data: Vec<u8>
}