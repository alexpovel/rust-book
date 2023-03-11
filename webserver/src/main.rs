use std::{
    collections::HashMap,
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    num::NonZeroUsize,
    thread,
    time::Duration,
};

use hello::{RequestResult, ThreadPool};

pub mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(NonZeroUsize::new(3).unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> RequestResult {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().ok_or(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "TCP stream ended unexpectedly.",
    ))??;

    let request = http::RequestHeader::try_from(request_line.as_str()).or(Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Unable to parse HTTP request.",
    )))?;

    let mut response = http::Response {
        version: http::Version::H1,
        status: http::StatusCode::OK,
        body: String::new(),
        headers: HashMap::new(),
    };

    let response = match request {
        http::RequestHeader { path, .. } if path == "/" => {
            response.body = fs::read_to_string("index.html").unwrap();
            response
        }
        http::RequestHeader { path, .. } if path == "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            response.body = fs::read_to_string("index.html").unwrap();
            response
        }
        _ => {
            response.body = fs::read_to_string("404.html").unwrap();
            response.status = http::StatusCode(404);
            response
        }
    };

    stream
        .write_all((String::from(response)).as_bytes())
        .unwrap();

    Ok(())
}
