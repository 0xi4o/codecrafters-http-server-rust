use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut stream_buffer = [0; 1024];
    stream.read(&mut stream_buffer).unwrap();
    // form request string from stream buffer
    let request = str::from_utf8(&stream_buffer).unwrap();
    let mut lines = request.split("\r\n");
    // get the first line of request by splitting the request string
    let first_line = lines.next().unwrap();
    // form method, path, and version from first line
    let mut first_line_parts = first_line.split_whitespace();
    let method = first_line_parts.next().unwrap().to_owned();
    let path = first_line_parts.next().unwrap().to_owned();
    // let version = first_line_parts.iter().next().unwrap().to_owned();

    println!("{method} {path}");

    let mut ok_response = "HTTP/1.1 200 OK\r\n".to_owned();
    if method == "GET" {
        // return 200 if path is "/"
        if path == "/" {
            ok_response.push_str("\r\n");
            stream.write(ok_response.as_bytes()).expect("unable to write to stream");
        }
        // if path is something else and starts with "/echo" or "/user-agent" return 200 with content type, length and body
        if let [_, root, pathname @ ..] = &path.split("/").collect::<Vec<&str>>()[..] {
            let content_type = "Content-Type: text/plain\r\n";
            let mut content = String::new();
            if root.to_owned() == "echo" {
                // calculate content length and set content
                content = pathname.join("/");
            } else if root.to_owned() == "user-agent" {
                // get the user agent string from request and set content
                let user_agent = lines.find(|item| item.starts_with("User-Agent")).unwrap().to_owned();
                content = user_agent.replace("User-Agent: ", "");
            } else {
                // return 404 for everything else
                stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
            }
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
        };
        // return 404 for everything else
        stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
    }
    stream.flush().unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
