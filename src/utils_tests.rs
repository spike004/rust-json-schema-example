use crate::utils;
use crate::models::AvailabilityRequest;
use crate::data::get_rdc_data;

#[test]
fn test_check_availability_valid_zip_and_product() {
    let zip = "10001";
    let product_id = "product1";
    let rdc_list = get_rdc_data();
    let result = utils::check_availability(zip, product_id, &rdc_list);
    assert_eq!(result, "Great news, all products are available to ship to you");
}

#[test]
fn test_check_availability_invalid_zip() {
    let zip = "99999";
    let product_id = "product1";
    let rdc_list = get_rdc_data();
    let result = utils::check_availability(zip, product_id, &rdc_list);
    assert_eq!(result, "We're sorry, but we are unable to ship to your location");
}

#[test]
fn test_check_availability_out_of_stock() {
    let zip = "20001";
    let product_id = "product1";
    let rdc_list = get_rdc_data();
    let result = utils::check_availability(zip, product_id, &rdc_list);
    assert_eq!(result, "We're sorry, but it looks like this product has sold out!");
}
