pub fn get_json_from_url(url: &str) -> Result<serde_json::Value, reqwest::Error> {
     match reqwest::blocking::get(url) {
        Ok(resp) => match resp.error_for_status() {
            Ok(resp_ok) => {
                match resp_ok.json::<serde_json::Value>() {
                    Ok(json) => Ok(json),
                    Err(decode_err) => {
                        println!("Failed to parse response: {:?} {:?}", decode_err, url);
                        Err(decode_err)
                    }
                }
            },
            Err(request_err) => {
                println!("Kubectl returned error: {:?}", request_err);
                Err(request_err)
            },
        },
        Err(connection_err) => {
            println!("Error while making connection: {:?}", connection_err);
            Err(connection_err)
        }
    }
}
