use esp_idf_sys::{EspError, esp_wifi_80211_tx};
use esp_idf_svc::wifi::WifiDeviceId;

// Bypass the check that prevents sending deauth packets
// Credit to: https://github.com/risinek/esp32-wifi-penetration-tool/blob/master/components/wsl_bypasser/README.md
#[no_mangle]
pub fn ieee80211_raw_frame_sanity_check() -> i32{
    return 0;
}

// Send a packet without restrictions
// As per [`esp_idf_sys::esp_wifi_internal_tx`](esp_idf_sys::esp_wifi_internal_tx)
pub fn send_freedom(device_id: WifiDeviceId, frame: &[u8]) -> Result<(), EspError> {
    use esp_idf_sys::esp;

    esp!(unsafe {
        esp_wifi_80211_tx(device_id.into(), frame.as_ptr() as *mut _, frame.len() as _, false)
    })
}