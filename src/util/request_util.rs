pub enum RequestError {
    Request(reqwest::Error),
    Parse(k8s_openapi::ResponseError),
}

pub fn get_response_from_url<T>(url: &str) -> Result<T, RequestError>
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
