use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("server listening at port 7878....");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established");
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("failed to establish connection : {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Recieved Request:\n{}", request);

            let status_line = "HTTP/1.1 200 OK";
            let contents = "<h1>Hello from your Rust server!</h1>";
            let content_length = format!("Content-Length: {}", contents.len());

            let response = format!("{}\r\n{}\r\n\r\n{}", status_line, content_length, contents);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("failed to read from stream: {}", e);
        }
    }
}
