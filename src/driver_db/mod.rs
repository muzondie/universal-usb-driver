use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceSignature {
    pub vendor_name: String,
    pub driver_package: Vec<u8>,
    pub checksum: u32,
}

lazy_static::lazy_static! {
    static ref DRIVER_DATABASE: HashMap<(u16, u16), DeviceSignature> = {
        let data = include_bytes!("../../drivers.bin");
        bincode::deserialize(data).unwrap()
    };
}

pub fn query_database(vendor_id: u16, product_id: u16) -> Option<DeviceSignature> {
    DRIVER_DATABASE.get(&(vendor_id, product_id)).cloned()
}

pub fn preload_drivers() {
    let _ = &*DRIVER_DATABASE;
}