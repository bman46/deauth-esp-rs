use std::{thread::sleep, time::Duration, collections::HashMap};

use embedded_svc::{http::{Method, server::HandlerError}, io::Write};
use esp_idf_svc::http::{server::EspHttpServer, server::Configuration};
use url::Url;

use self::html_formatting::{templated_html::templated_html, templated_table, templated_link::templated_link};
use crate::{http_server::templated_table::templated_table, WIFI, mac::{format_mac, decode_mac}, wifi_deauth::deauth};

mod html_formatting;

pub fn start_http_server(){
    // Set the HTTP server
    let mut server = EspHttpServer::new(&Configuration{
        ..Default::default()
    }).unwrap();
    // http://<sta ip>/ handler
    server.fn_handler("/", Method::Get, |request| {
        let html = templated_html("Home" ,format!("Welcome!<br />{}", templated_link("WiFi Scan", "/scan")));
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    }).unwrap();

    // Scan page
    server.fn_handler("/scan", Method::Get, |request| {
        let scan_result = WIFI.lock().unwrap().scan();
        let mut contents = "".to_owned();
        match scan_result {
            Ok(aps) => {
                let mut vec1 = vec!["SSID".to_string(), "Channel".to_string(), "Strength".to_string(), "BSSID".to_string(), "Actions".to_string()];
                for ap in aps{
                    vec1.push(ap.ssid.to_string());
                    vec1.push(ap.channel.to_string());
                    vec1.push(ap.signal_strength.to_string());
                    vec1.push(format_mac(ap.bssid, true));
                    vec1.push(templated_link("Deauth", format!("/deauth?chan={}&bssid={}", ap.channel, format_mac(ap.bssid, false))))
                }
                let table_res = templated_table(vec1, 4);
                match table_res
                {
                    Ok(str) => {
                        contents.push_str("<h1>Scan Results</h1>");
                        contents.push_str(&str);
                    },
                    Err(e) => {
                        contents.push_str(&format!("<p>Failed to parse to table: {}</p>", e));
                    }
                }
            },
            Err(error) => {
                contents = format!("Error scanning: {}", error.code());
            }
        }

        let html = templated_html("Scan Result", contents);
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    }).unwrap();

    //deauth function
    //path: /deauth?chan=1&bssid=123
    server.fn_handler("/deauth", Method::Get, |request| {
        let mut url = Url::parse(request.uri()).unwrap();
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let chan = params.get("chan").unwrap().as_bytes().first();
        let mac_result =  decode_mac(params.get("bssid").unwrap());

        let mut config = WIFI.lock().unwrap().get_configuration().unwrap();
        match chan {
            Some(chan) => {
                config.as_mixed_conf_mut().0.channel = Some(chan.to_owned());
            },
            None => {
                let html = templated_html("Deauth Result", "Failed to process the channel variable.");
                let mut response = request.into_status_response(400)?;
                response.write_all(html.as_bytes())?;

                return Err(HandlerError::new("Failed to process channel."));
            }
        }

        WIFI.lock().unwrap().set_configuration(&config).unwrap();

        match mac_result {
            Ok(mac) =>{
                let result = deauth(mac);
                if result.is_err(){
                    let html = templated_html("Deauth Result", "Deauth failed.");
                    let mut response = request.into_status_response(400)?;
                    response.write_all(html.as_bytes())?;
    
                    return Err(HandlerError::new("Failed to process mac."));
                }
            },
            Err(_) => {
                let html = templated_html("Deauth Result", "Invalid MAC addr.");
                let mut response = request.into_status_response(400)?;
                response.write_all(html.as_bytes())?;

                return Err(HandlerError::new("Failed to process mac."));
            }
        }

        let html = templated_html("Deauth Result", format!("Last: {}, First: {}", chan_process.unwrap(), format_mac(mac_result.unwrap(), true)));
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;

        return Ok(());
    }).unwrap();

    // Hold thread open:
    loop {
        // Hold HTTP server open:
        sleep(Duration::from_millis(1000));
    }
}