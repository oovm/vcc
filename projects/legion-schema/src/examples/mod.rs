use std::str::FromStr;
use url::Url;

pub fn homepage_example() -> Vec<Url> {
    vec![Url::from_str("https://github.com/").unwrap()]
}
pub fn repository_example() -> Vec<Url> {
    vec![Url::from_str("https://github.com/").unwrap()]
}

pub fn document_example() -> Vec<Url> {
    vec![Url::from_str("https://docs.rs/").unwrap()]
}
