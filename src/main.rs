use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::{env, fs, str, thread};
use std::path::Path;

fn handle_client(mut stream: TcpStream, dir: String) {
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

    println!("{method} {path}");

    let mut ok_response = "HTTP/1.1 200 OK\r\n".to_owned();
    if method == "GET" {
        // return 200 if path is "/"
        if path == "/" {
            ok_response.push_str("\r\n");
            stream.write(ok_response.as_bytes()).expect("unable to write to stream");
        } else {
            // if path is something else and starts with "/echo" or "/user-agent" return 200 with content type, length and body
            if let [_, root, pathname @ ..] = &path.split("/").collect::<Vec<&str>>()[..] {
                let mut content_type = "Content-Type: text/plain\r\n";
                let mut content = String::new();
                if root.to_owned() == "echo" {
                    // calculate content length and set content
                    content = pathname.join("/");
                } else if root.to_owned() == "user-agent" {
                    // get the user agent string from request and set content
                    let user_agent = lines.find(|item| item.starts_with("User-Agent")).unwrap().to_owned();
                    content = user_agent.replace("User-Agent: ", "");
                } else if root.to_owned() == "files" {
                    // handle reading a file and returning the contents in the response
                    let filename = pathname.join("");
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
                } else {
                    // return 404 for everything else
                    stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
                    stream.flush().unwrap();
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
                stream.flush().unwrap();
            };
        }
        // return 404 for everything else
        stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
    } else if method == "POST" {
        let body = lines.last().unwrap();
        println!("{body}");
        if let [_, root, pathname @ ..] = &path.split("/").collect::<Vec<&str>>()[..] {
            if root.to_owned() == "files" {
                let filename = pathname.join("");
                let file_path = format!("{}/{}", dir, filename);
                println!("writing to {file_path}");
                let _ = write_to_file(&dir, body, file_path);
                ok_response = "HTTP/1.1 201 OK\r\n".to_owned();
                stream.write(ok_response.as_bytes()).expect("unable to write to stream");
                stream.flush().unwrap();
            }
        }
    } else {
        // return 404 for everything else
        stream.write("HTTP/1.1 404 OK\r\n\r\n".as_bytes()).expect("unable to write to stream");
    }
    stream.flush().unwrap();
}

fn write_to_file(dir: &String, body: &str, file_path: String) -> anyhow::Result<()> {
    if Path::new(&dir).is_dir() {
        println!("dir exists -> writing to file");
        fs::write(file_path, body.as_bytes())?;
    } else {
        println!("dir doesn't exist -> creating dir");
        fs::create_dir_all(dir)?;
        println!("created dir -> writing to file");
        fs::write(file_path, body.as_bytes())?;
    }
    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut dir = "".to_owned();
    if let [flag, d] = &args[..] {
        if flag == "--directory" {
            dir = d.to_owned();
        }
    }

    println!("directory: {dir}");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = dir.clone();
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_client(stream, directory);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
