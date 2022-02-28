use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self { address: address }
  }
  pub fn run(self) {
    println!("Listening on {}", self.address);

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 2048];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!(request);
                  let response = Response::new(
                    StatusCode::Ok,
                    Some("<h1>COCONUT SERVER WORKS!</h1".to_string()),
                  );
                  write!(stream, "{}", response);
                }
                Err(error) => {
                  println!("Failed to parse a request: {}", error);
                }
              }
            }
            Err(error) => {
              println!("Failed to read from connection: {}", error);
            }
          }
        }
        Err(error) => {
          println!("Failed to establish a connection: {}", error);
        }
      }
    }
  }
}
