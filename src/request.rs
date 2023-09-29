use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    pub fn from(method_str: &str) -> Self {
        match method_str {
            "GET" => Self::GET,
            "POST" => Self::POST,
            m => panic!("unsupported http method: {m}")
        }
    }
}

#[derive(Debug)]
pub struct HttpUrl {
    pub root: String,
    pub pathname: Option<String>,
}

impl HttpUrl {
    pub fn from(path_str: &str) -> Self {
        let mut http_root = String::new();
        return if let [_, root, pathname @ ..] = &path_str.split("/").collect::<Vec<&str>>()[..] {
            http_root = root.to_string();
            let http_path = pathname.join("/").to_owned();
            HttpUrl {
                root: http_root,
                pathname: Some(http_path),
            }
        } else {
            HttpUrl {
                root: http_root,
                pathname: None,
            }
        };
    }
}

pub struct HttpRequest {
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
    pub method: HttpMethod,
    pub url: HttpUrl,
}

impl HttpRequest {
    pub async fn parse(req_str: &str) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut lines = req_str.split("\r\n");
        let first_line = lines.next().unwrap();
        let mut first_line_parts = first_line.split_whitespace();
        let method_str = first_line_parts.next().unwrap();
        let method = HttpMethod::from(method_str);
        let url_str = first_line_parts.next().unwrap();
        let url = HttpUrl::from(url_str);

        let request_headers = lines.clone().take_while(|line| !line.is_empty());

        for line in request_headers {
            if let [key, value] = &line.split(": ").collect::<Vec<&str>>()[..] {
                let header_key = key.to_string();
                let header_value = value.to_string();
                headers.insert(header_key, header_value);
            }
        }

        match method {
            HttpMethod::POST => {
                let body = lines.skip_while(|line| !line.is_empty()).join("\r\n");
                HttpRequest {
                    body: Some(body),
                    headers,
                    method,
                    url,
                }
            }
            HttpMethod::GET => {
                HttpRequest {
                    body: None,
                    headers,
                    method,
                    url,
                }
            }
        }
    }
}

