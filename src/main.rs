use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::fs;

fn handle_connection(mut stream: TcpStream)  {
    let mut streng = String::new();
    match stream.read_to_string(&mut streng) {
        Ok(s) => {
            println!("{}", s);
        }
        Err(e) => {
            panic!("{}",e);
        }
    }
        ;
    // let read_file = fs::read_to_string("index.html");
    // match read_file {
    //     Ok(file) => { stream.write(file.as_bytes()).unwrap(); }
    //     Err(_) => { panic!("Could not read file"); }
    // }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                panic!("{}",e);
            }
        }
        println!("Connection established!");
    }
}
