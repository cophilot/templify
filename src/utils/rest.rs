use reqwest::blocking::Client;

/// Make a REST call to the given URL accepting JSON and return the response.
pub fn json_call(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = Client::new();
    client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
}
