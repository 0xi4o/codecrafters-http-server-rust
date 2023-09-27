use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut stream_buffer = [0; 1024];
    stream.read(&mut stream_buffer).unwrap();
    // form request string from stream buffer
    let request = str::from_utf8(&stream_buffer).unwrap();
    // get the first line of request by splitting the request string
    let first_line = request.split("\r\n").collect::<Vec<&str>>().first().unwrap().to_owned();
    // form method, path, and version from first line
    let mut first_line_parts = first_line.split_whitespace();
    let method = first_line_parts.next().unwrap().to_owned();
    let path = first_line_parts.next().unwrap().to_owned();
    // let version = first_line_parts.iter().next().unwrap().to_owned();

    println!("{method} {path}");

    let mut ok_response = "HTTP/1.1 200 OK\r\n".to_owned();
    if method == "GET" {
        if let [_, root, pathname @ ..] = &path.split("/").collect::<Vec<&str>>()[..] {
            // return 200 if path is "/" and 404 for everything else
            if path == "/" {
                ok_response.push_str("\r\n");
                stream.write(ok_response.as_bytes()).expect("unable to write to stream");
            }
            if root.to_owned() == "echo" {
                let content_type = "Content-Type: text/plain\r\n";
                // calculate content length and set value for response string
                let pathname = &pathname.join("/");
                let mut content_length = "Content-Length: ".to_owned();
                content_length.push_str(&pathname.len().to_string());
                content_length.push_str("\r\n");

                // form response string
                ok_response.push_str(content_type);
                ok_response.push_str(&content_length);
                ok_response.push_str("\r\n");
                ok_response.push_str(&pathname);

                stream.write(ok_response.as_bytes()).expect("unable to write to stream");
            }
            stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
        };
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
