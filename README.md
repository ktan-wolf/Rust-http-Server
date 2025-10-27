# 🦀 Simple Rust TCP Web Server

A minimal HTTP server written in pure **Rust**, using only the standard library (`std::net` and `std::io`).  
This project demonstrates how to handle TCP connections, read HTTP requests, and send simple HTTP responses — all manually, without any external web frameworks.

---

## 🚀 Features

- Uses `TcpListener` and `TcpStream` from the Rust standard library  
- Handles multiple client connections using threads (`std::thread::spawn`)  
- Parses and prints incoming HTTP requests to the console  
- Sends a valid HTTP response (`HTTP/1.1 200 OK`)  
- Fully self-contained — **no external dependencies**

---

## 🧠 How It Works

1. The server binds to `127.0.0.1:7878` and listens for incoming TCP connections.  
2. When a client connects (for example, a browser), a new thread is spawned to handle that connection.  
3. The server reads up to 1024 bytes from the TCP stream.  
4. The raw request is printed to the terminal.  
5. A simple HTTP response with HTML content is sent back to the client.  

---

## 📦 Code Overview

```rust
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
```

---

## ⚙️ Running the Server

### 🧩 Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (version 1.60+ recommended)

### ▶️ Run the server

```bash
cargo run
```

The server will start and listen on port `7878`:
```
server listening at port 7878....
```

### 🌐 Test it in your browser

Visit:  
👉 [http://127.0.0.1:7878](http://127.0.0.1:7878)

You should see:
```
Hello from your Rust server!
```

Your terminal will print the raw HTTP request that your browser sent.

---

## 🧵 Multithreading

Each connection is handled in a new thread:
```rustt
hread::spawn(move || {
    handle_connection(stream);
});
```
This allows multiple clients to connect simultaneously without blocking the main thread.

---

## 🧰 Concepts Demonstrated

- TCP networking in Rust  
- Basic HTTP protocol structure  
- Threaded request handling  
- Stream reading/writing  
- Manual request/response formatting

---

## 🧪 Example Output

**Terminal:**
```
server listening at port 7878....
Connection established
Received Request:
GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 ...
```

**Browser:**
```html
<h1>Hello from your Rust server!</h1>
```

---

## 📄 License
This project is licensed under the **MIT License**.  
Feel free to use and modify it for your own experiments or learning.

---

## 💡 Inspiration
This example is inspired by the “Building a Multithreaded Web Server” chapter from *The Rust Programming Language* book (a.k.a. “The Rust Book”).
