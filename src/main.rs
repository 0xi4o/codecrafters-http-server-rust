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
    // split the first line to access its parts like method, path, and version
    let first_line_parts = first_line.split_whitespace().collect::<Vec<&str>>();
    // return 200 if path is "/" and 404 for everything else
    if first_line_parts[1] == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
    } else {
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
