use std::{thread::sleep, time::Duration};

use embedded_svc::{http::Method, io::Write};
use esp_idf_svc::http::{server::EspHttpServer, server::Configuration};

use self::html_formatting::{templated_html::templated_html, templated_table};
use crate::{http_server::templated_table::templated_table, WIFI};

mod html_formatting;

pub fn start_http_server(){
    // Set the HTTP server
    let mut server = EspHttpServer::new(&Configuration{
        ..Default::default()
    }).unwrap();
    // http://<sta ip>/ handler
    server.fn_handler("/", Method::Get, |request| {
        let html = templated_html("Home" ,"Welcome!");
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    }).unwrap();

    server.fn_handler("/scan", Method::Get, |request| {
        let scan_result = WIFI.lock().unwrap().scan();
        let mut contents = "".to_owned();
        match scan_result {
            Ok(aps) => {
                let mut vec1 = vec!["SSID".to_string(), "Channel".to_string(), "BSSID".to_string()];
                for ap in aps{
                    vec1.push(ap.ssid.to_string());
                    vec1.push(ap.channel.to_string());
                    vec1.push(format!("{:02X?}", ap.bssid));
                }
                let table_res = templated_table(vec1, 3);
                match table_res{
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

    // Hold thread open:
    loop {
        // Hold HTTP server open:
        sleep(Duration::from_millis(1000));
    }
}