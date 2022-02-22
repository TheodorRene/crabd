use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Write;
use std::io::Read;
use std::fs;
use std::str;

use chrono::prelude::*;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut streng = [0; 4];
    let mut vec = Vec::new();
   // let x = Local::now();
    while streng != [13,10,13,10] {
        stream.read(&mut streng).unwrap();
        vec.extend(streng);
    }
    println!("{}", str::from_utf8(&vec).unwrap());
        
    let read_file = fs::read_to_string("index.html");
    match read_file {
         Ok(file) => { stream.write(file.as_bytes()).unwrap(); }
         Err(_) => { panic!("Could not read file"); }
    }
    stream.shutdown(Shutdown::Both)?;
    return Ok(());
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).unwrap;
            }
            Err(e) => {
                panic!("{}",e);
            }
        }
        println!("Connection established!");
    }
}
