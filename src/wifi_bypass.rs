//! Bypass the check that prevents sending deauth packets
//! Credit to: https://github.com/risinek/esp32-wifi-penetration-tool/blob/master/components/wsl_bypasser/README.md
#[no_mangle]
pub fn ieee80211_raw_frame_sanity_check() -> i32{
    return 0;
}