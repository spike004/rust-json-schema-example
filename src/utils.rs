use crate::models::{AvailabilityRequest, RDC};
use crate::data::get_rdc_data;
use warp::Rejection;

pub async fn handle_availability_request(req: AvailabilityRequest) -> Result<impl warp::Reply, Rejection> {
    let rdc_list = get_rdc_data();
    let response = check_availability(&req.zip, &req.product_id, &rdc_list);
    Ok(warp::reply::json(&response))
}

pub(crate) fn check_availability(zip: &str, product_id: &str, rdc_list: &[RDC]) -> String {
    for rdc in rdc_list {
        if rdc.serviced_zip_codes.contains(&zip.to_string()) {
            match rdc.inventory.get(product_id) {
                Some(&quantity) if quantity > 0 => return "Great news, all products are available to ship to you".to_string(),
                _ => return "We're sorry, but it looks like this product has sold out!".to_string(),
            }
        }
    }
    "We're sorry, but we are unable to ship to your location".to_string()
}
