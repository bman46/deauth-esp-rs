use std::{thread::sleep, time::Duration};

use embedded_svc::{http::Method, io::Write};
use esp_idf_svc::{http::{server::EspHttpServer, server::Configuration}, wifi::{BlockingWifi, EspWifi}};

use self::html_formatting::{templated_html::templated_html, templated_table};
use crate::http_server::templated_table::templated_table;

mod html_formatting;

pub fn start_http_server(wifi: &BlockingWifi<EspWifi<'_>>){
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

        let scan_result = wifi.scan();
        let mut contents = "".to_owned();
        match scan_result {
            Ok(aps) => {
                let mut vec1 = vec!["SSID", "Channel"];
                for ap in aps{
                    vec1.push(&ap.ssid);
                    vec1.push(&ap.channel.to_string());
                }
                let table_res = templated_table(vec1, 2);
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