

use url::Url;

pub fn validate_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(parsed_url) => {
            let scheme = parsed_url.scheme();
            scheme == "http" || scheme == "https"
        }
        Err(_) => false,
    }
}