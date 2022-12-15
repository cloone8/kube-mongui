pub enum RequestError {
    Request(reqwest::Error),
    Parse(k8s_openapi::ResponseError),
}

pub fn get_response_from_url<T>(url: &str) -> Result<T, RequestError>
    where T: k8s_openapi::Response
{
    log::info!("Requesting (typed): {}", url);

    let response = get_response_from_url_unlogged::<T>(url);

    if response.is_err() {
        log::warn!("Request failed: {}", url);
    } else {
        log::debug!("Request succeeded: {}", url);
    }

    response
}

#[inline]
fn get_response_from_url_unlogged<T>(url: &str) -> Result<T, RequestError>
    where T: k8s_openapi::Response
{
    let response = match reqwest::blocking::get(url) {
        Ok(it) => it,
        Err(err) => return Err(RequestError::Request(err)),
    };

    match T::try_from_parts(response.status(), response.bytes().unwrap().as_ref()) {
        Ok((parsed, _)) => Ok(parsed),
        Err(parse_err) => Err(RequestError::Parse(parse_err)),
    }
}

pub fn attempt_as_json<T>(url: &str) -> Result<Vec<T>, RequestError>
    where T: serde::de::DeserializeOwned
{
    log::info!("Requesting (json): {}", url);

    let response = attempt_as_json_unlogged::<T>(url);

    if response.is_err() {
        log::warn!("Request failed: {}", url);
    } else {
        log::debug!("Request succeeded: {}", url);
    }

    response
}

#[inline]
fn attempt_as_json_unlogged<T>(url: &str) -> Result<Vec<T>, RequestError>
    where T: serde::de::DeserializeOwned
{
    let response = match reqwest::blocking::get(url) {
        Ok(it) => match it.json::<serde_json::Value>() {
            Ok(json) => json,
            Err(err) => return Err(RequestError::Request(err)),
        }
        Err(err) => return Err(RequestError::Request(err)),
    };

    let items = response["items"].as_array().unwrap().iter()
        .map(|item| serde_json::from_value::<T>(item.clone()).unwrap())
        .collect::<Vec<T>>();

    Ok(items)
}
