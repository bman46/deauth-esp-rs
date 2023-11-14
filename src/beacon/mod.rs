
pub fn beacon(){
    
}

fn beacon_frame_builder(bssid: [u8; 6]) -> [u8; 26] {
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