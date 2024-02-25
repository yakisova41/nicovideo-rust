use reqwest::header::HeaderMap;
use std::collections::HashMap;


pub type ParsedHeaderMap = HashMap<String, String>;

pub fn parse_headers(headers: &HeaderMap) -> ParsedHeaderMap {
    let mut parsed: ParsedHeaderMap = HashMap::new();

    for i in headers.iter() {
        parsed.insert(i.0.to_string(), reqwest::header::HeaderValue::to_str(i.1).unwrap().to_string());
    }

    parsed
}