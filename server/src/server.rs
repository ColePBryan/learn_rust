use std::{convert::TryFrom, net::TcpListener, io::Read};
use crate::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self{
        Self{address}
    }

    pub fn run(self){
        println!("listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            println!("Parsing request...");
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => { 
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse a request {}", e)
                            }

                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
                
            }
        }
    }
}