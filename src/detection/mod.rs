use windows::Win32::Devices::Usb;
use super::driver_db::DeviceSignature;

pub struct HardwareID {
    vendor_id: u16,
    product_id: u16,
    revision: u16,
}

pub async fn scan_connected_devices() -> Vec<HardwareID> {
    let mut devices = Vec::new();
    
    unsafe {
        let mut device_info = Usb::USB_DEVICE_INFO::default();
        let mut hub_handle = Usb::USB_HUB_INFORMATION::default();
        
        if Usb::GetUsbHubInformation(std::ptr::null_mut(), &mut hub_handle).is_ok() {
            let mut index = 0;
            while Usb::GetDeviceInformation(hub_handle.HubHandle, index, &mut device_info).is_ok() {
                devices.push(HardwareID {
                    vendor_id: device_info.VendorId,
                    product_id: device_info.ProductId,
                    revision: device_info.Revision,
                });
                index += 1;
            }
        }
    }
    
    devices
}

pub fn match_hardware_to_driver(id: &HardwareID) -> Option<DeviceSignature> {
    driver_db::query_database(id.vendor_id, id.product_id)
}