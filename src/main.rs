use std::fmt;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;

use chrono::prelude::*;

#[derive(Debug)]
enum HeaderField {
    Date,
    Server,
    LastModified,
    ETag,
    AcceptRanges,
    ContentLength,
}

impl fmt::Display for HeaderField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            HeaderField::Date => "Date".to_string(),
            HeaderField::Server => "Server".to_string(),
            HeaderField::LastModified => "Last-Modified".to_string(),
            HeaderField::ETag => "Etag".to_string(),
            HeaderField::AcceptRanges => "Accept-Ranges".to_string(),
            HeaderField::ContentLength => "Content-Length".to_string(),
        };
        write!(f, "{}", string)
    }
}

struct HeaderValue(HeaderField, String);

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let HeaderValue(header, val) = self;
        write!(f, "{}:{}", header.to_string(), val)
    }
}

fn agg_headers(list_of_headers: Vec<HeaderValue>) -> String {
    let mut result = String::new();
    list_of_headers.iter().for_each(|item| {
        let s = format!("{}\n", item.to_string());
        result.push_str(&s);
    });
    result
}

fn get_header(body: &String) -> String {
    let date = HeaderValue(HeaderField::Date, Local::now().to_rfc2822());
    let server = HeaderValue(HeaderField::Server, "Crabd 0.1".to_string());
    let contentlength = HeaderValue(HeaderField::ContentLength, body.len().to_string());
    let headers = agg_headers(vec![date, server, contentlength]);
    let verb = "HTTP/1.1 200 OK".to_string(); // TODO Support other Verbs
    format!("{}\n{}\n{}", verb, headers, body) // TODO Turn into a struct with a to_display
                                               //TODO add better logging
}

fn get_uri(req: &str) -> String {
    let http_verb : Vec<&str> = req.lines().next().unwrap().split(" ").collect();
    http_verb[1].to_string()
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut streng = [0; 4];
    let mut vec = Vec::new();

    //https://thepacketgeek.com/rust/tcpstream/reading-and-writing/
    // TODO reading logic
    // Write now it will read the same 4 bytes forever if it never ends with CLRF. 
    // This is inconvenient. Can also check for empty bytes
    while streng != [13, 10, 13, 10] {
        println!("{:?}", &streng);
        stream.read(&mut streng).unwrap();
        vec.extend(streng);
    }
    let req = str::from_utf8(&vec).unwrap();
    let uri = get_uri(req);
    let read_file = fs::read_to_string("index.html");
    match read_file {
        Ok(file) => {
            let header = get_header(&file);
            println!("{}", &header);
            stream.write(header.as_bytes()).unwrap();
        }
        Err(_) => {
            panic!("Could not read file");
        }
    }
    stream.shutdown(Shutdown::Both)?;
    return Ok(());
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).unwrap();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

// Tests
// TODO move to another file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn contains_http_verb() {
        let x = get_header(&"".to_string());
        assert!(x.contains("HTTP"))
    }
}
