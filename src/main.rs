use std::sync::Mutex;

use embedded_svc::wifi::{Configuration, ClientConfiguration, AccessPointConfiguration, AuthMethod};
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    wifi::{EspWifi, BlockingWifi},
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use once_cell::sync::Lazy;

use crate::http_server::start_http_server;

mod wsl_bypasser;
mod wifi_deauth;
mod http_server;
mod mac;

static WIFI: Lazy<Mutex<BlockingWifi<EspWifi<'_>>>> = Lazy::new(|| {
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    
    let wifi: BlockingWifi<EspWifi<'_>> = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    ).unwrap();

    Mutex::new(wifi)
});

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting...");

    let wifi_config = Configuration::Mixed(
        ClientConfiguration{
            ..Default::default()
        }, 
        AccessPointConfiguration{
            ssid: "Tool MGMT".into(),
            password: "password123".into(),
            auth_method: AuthMethod::WPA2Personal,
            ..Default::default()
        }
    );

    WIFI.lock().unwrap().set_configuration(&wifi_config).unwrap();

    WIFI.lock().unwrap().start().unwrap();

    start_http_server();
}