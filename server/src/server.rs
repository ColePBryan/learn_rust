use std::{convert::TryFrom, net::{TcpListener, TcpStream}, io::Read, thread};
use crate::http::{Request, Response, StatusCode};

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

        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    thread::spawn(||{
                        Server::handle_connection(stream);
                    });
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }

    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                println!("Parsing request...");
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => { 
                        dbg!(request);
                        Response::new(StatusCode::Ok, Some("<h1> IT WORKS STILL!!!! </h1>".to_string()))
                    },
                    Err(e) => {
                        println!("Failed to parse a request {}", e);
                        Response::new(StatusCode::BadRequest, None)
                    }
                };
                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }

            }
            Err(e) => println!("Failed to read from connection: {}", e)
        }
            
    }
}