pub fn get_json_from_url(url: &str) -> Option<serde_json::Value> {
     match reqwest::blocking::get(url.clone()) {
        Ok(resp) => match resp.error_for_status() {
            Ok(resp_ok) => {
                match resp_ok.json::<serde_json::Value>() {
                    Ok(json) => Some(json),
                    Err(decode_err) => {
                        println!("Failed to parse response: {:?} {:?}", decode_err, url.clone());
                        None
                    }
                }
            },
            Err(request_err) => {
                println!("Kubectl returned error: {:?}", request_err);
                None
            },
        },
        Err(connection_err) => {
            println!("Error while making connection: {:?}", connection_err);
            None
        }
    }
}
