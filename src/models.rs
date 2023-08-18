use std::collections::HashMap;
use serde::Deserialize;


pub struct RDC {
    pub id: u32,
    pub serviced_zip_codes: Vec<String>,
    pub inventory: HashMap<String, u32>,
}
#[derive(Deserialize)]
pub struct AvailabilityRequest {
    pub zip: String,
    pub product_id: String,
}
