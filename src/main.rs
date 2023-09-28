use std::io::{BufRead, BufReader, Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{env, fs, str, thread};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
}

impl HttpMethod {
    fn from(method_str: &str) -> Self {
        match method_str {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => panic!("invalid method")
        }
    }
}

async fn handle_client(mut stream: TcpStream, dir: String) {
    let buf_reader = BufReader::new(&mut stream);
    // form request string from stream buffer
    let buf_lines = &buf_reader.lines().map(|line| line.expect("could not read line")).collect::<Vec<_>>();
    let headers = buf_lines.iter().take_while(|line| !line.is_empty()).collect::<Vec<_>>();
    let body_arr = buf_lines.iter().skip_while(|line| !line.is_empty()).map(|line| line.to_owned()).collect::<Vec<_>>();
    let body = body_arr.join("\r\n");
    // get the first line of headers by splitting the request string
    let first_line = headers.iter().next().unwrap();
    // form method, path, and version from first line
    let mut first_line_parts = first_line.split_whitespace();
    let method_str = first_line_parts.next().unwrap();
    let method = HttpMethod::from(method_str);
    let path = first_line_parts.next().unwrap().to_owned();
    println!("{:?} {path}", method);
    let mut ok_response = "HTTP/1.1 200 OK\r\n".to_owned();
    let mut http_path_root = String::new();
    let mut http_path = String::new();
    if let [_, root, pathname @ ..] = &path.split("/").map(|part| part.to_owned()).collect::<Vec<String>>()[..] {
        http_path_root = root.to_owned();
        http_path = pathname.join("/");
    }
    match method {
        HttpMethod::GET => {
            let mut content_type = "Content-Type: text/plain\r\n";
            let mut content = String::new();
            // if path is something else and starts with "/echo" or "/user-agent" return 200 with content type, length and body
            if http_path_root == "echo" {
                // calculate content length and set content
                content = http_path;
                send_response(&mut stream, &mut ok_response, content_type, &mut content);
            } else if http_path_root == "user-agent" {
                // get the user agent string from request and set content
                let user_agent = headers.iter().find(|item| item.starts_with("User-Agent")).unwrap().to_owned();
                content = user_agent.replace("User-Agent: ", "");
                send_response(&mut stream, &mut ok_response, content_type, &mut content);
            } else if http_path_root == "files" {
                // handle reading a file and returning the contents in the response
                let filename = http_path;
                let file_path = format!("{}/{}", dir, filename);
                if Path::new(&dir).is_dir() {
                    if Path::new(&file_path).is_file() {
                        content_type = "Content-Type: application/octet-stream\r\n";
                        content = fs::read_to_string(file_path).unwrap();
                    } else {
                        stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
                    }
                } else {
                    stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
                }
                send_response(&mut stream, &mut ok_response, content_type, &mut content);
            } else if http_path_root == "" {
                // return 200 if path is "/"
                ok_response.push_str("\r\n");
                stream.write(ok_response.as_bytes()).expect("unable to write to stream");
                stream.flush().unwrap();
            } else {
                // return 404 for everything else
                stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
                stream.flush().unwrap();
            }
        }
        HttpMethod::POST => {
            if http_path_root.to_owned() == "files" {
                let filename = http_path;
                let file_path = format!("{}{}", dir, filename);
                println!("writing to {file_path}");
                let _ = write_to_file(body, file_path);
                ok_response = "HTTP/1.1 201 CREATED\r\n".to_owned();
                stream.write(ok_response.as_bytes()).expect("unable to write to stream");
                stream.flush().unwrap();
            }
        }
    }
}

fn send_response(stream: &mut TcpStream, ok_response: &mut String, content_type: &str, content: &mut String) {
// form content length and response from content
    let mut content_length = "Content-Length: ".to_owned();
    content_length.push_str(&content.len().to_string());
    content_length.push_str("\r\n");

    // form response string
    ok_response.push_str(content_type);
    ok_response.push_str(&content_length);
    ok_response.push_str("\r\n");
    ok_response.push_str(&content);

    stream.write(ok_response.as_bytes()).expect("unable to write to stream");
    stream.flush().unwrap();
}

fn write_to_file(body: String, file_path: String) -> anyhow::Result<()> {
    println!("writing to {file_path}: {body}");
    let mut buffer = File::create(file_path)?;
    buffer.write_all(body.as_bytes())?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args = env::args().skip(2).collect::<Vec<String>>();
    let mut dir = "".to_owned();
    if let [flag, d] = &args[..] {
        if flag == "--directory" {
            dir = d.to_owned();
        }
    }

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = dir.clone();
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_client(stream, directory).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
