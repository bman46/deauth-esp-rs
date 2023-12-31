pub fn format_mac(mac: [u8; 6], use_colon: bool) -> String{
    let mut mac_string = "".to_string();
    for segment in mac
    {
        mac_string.push_str(&hex::encode([segment]));
        if use_colon
        {
            mac_string.push(':');
        }
    }
    if use_colon
    {
        mac_string.pop();
    }

    return mac_string;
}

pub fn decode_mac(mac_str: &str) -> Result<[u8; 6], &str>{
    let clean_str = mac_str.replace(":", "");
    let mut mac:[u8; 6] = [0,0,0,0,0,0];

    for i in 0..6{
        let arr = hex::decode(&clean_str).unwrap();
        let current = arr.get(i);
        match current{
            Some(val) => {
                mac[i] = val.clone();
            },
            None => {
                return Err("MAC address parsing error.");
            }
        }
    }

    return Ok(mac);
}