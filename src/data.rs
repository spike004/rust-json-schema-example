use crate::models::RDC;

pub fn get_rdc_data() -> Vec<RDC> {
    vec![
        RDC {
            id: 1,
            serviced_zip_codes: vec!["10001".to_string(), "10002".to_string()],
            inventory: [("product1".to_string(), 10), ("product2".to_string(), 5)].iter().cloned().collect(),
        },
        RDC {
            id: 2,
            serviced_zip_codes: vec!["20001".to_string()],
            inventory: [("product1".to_string(), 0), ("product2".to_string(), 8)].iter().cloned().collect(),
        }
    ]
}
