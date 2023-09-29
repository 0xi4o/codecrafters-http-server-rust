mod request;

use std::str;
// Uncomment this block to pass the first stage
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::request::{HttpMethod, HttpRequest};

async fn process_request(mut stream: TcpStream) {
    let mut stream_buffer = [0; 1024];
    stream.read(&mut stream_buffer).await.unwrap();
    // form request string from stream buffer
    let req_str = str::from_utf8(&stream_buffer).unwrap();
    let request = HttpRequest::parse(req_str).await;
    let url = &request.url;
    let root = &url.root;
    let pathname = &url.pathname.clone().unwrap();
    let path_str = if pathname != "" {
        format!("/{root}/{pathname}")
    } else {
        format!("/{root}")
    };
    // get the first line of request by splitting the request string
    let mut ok_response = "HTTP/1.1 200 OK\r\n".to_owned();
    match request.method {
        HttpMethod::GET => {
            let content_type = "Content-Type: text/plain\r\n";
            if request.url.root == "" {
                ok_response.push_str("\r\n");
                println!("{:?} {path_str} - 200 OK", request.method);
                let _ = stream.write(ok_response.as_bytes()).await;
            } else if request.url.root == "echo" {
                // get pathname from request and send response
                let mut content = request.url.pathname.unwrap();
                println!("{:?} {path_str} - 200 OK", request.method);
                send_response(&mut stream, ok_response, &mut content, content_type).await;
            } else if request.url.root == "user-agent" {
                // get the user agent string from request headers and send response
                let user_agent = request.headers.get("User-Agent").unwrap();
                let mut content = user_agent.to_string();
                println!("{:?} {path_str} - 200 OK", request.method);
                send_response(&mut stream, ok_response, &mut content, content_type).await;
            } else {
                println!("{:?} {path_str} - 404 Not Found", request.method);
                let _ = stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).await;
            }
        }
        HttpMethod::POST => {
            todo!();
        }
    }
}

async fn send_response(stream: &mut TcpStream, mut ok_response: String, content: &mut String, content_type: &str) {
    let mut content_length = "Content-Length: ".to_owned();
    content_length.push_str(&content.len().to_string());
    content_length.push_str("\r\n");
    // form response string
    ok_response.push_str(content_type);
    ok_response.push_str(&content_length);
    ok_response.push_str("\r\n");
    ok_response.push_str(&content);
    let _ = stream.write(ok_response.as_bytes()).await;
}

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process_request(socket).await;
        });
    }
}