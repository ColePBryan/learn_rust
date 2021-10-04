use std::{convert::TryFrom, net::{TcpListener, TcpStream}, io::Read, thread};
use crate::http::{ParseError, Request, Response, StatusCode};
use crate::lib::ThreadPool;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self{
        Self{address}
    }

    pub fn run(self, mut handler: impl Handler){
        println!("listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    pool.execute(||{
                        Server::handle_connection(stream, handler);
                    });
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }

    }

    pub fn handle_connection(mut stream: TcpStream, mut handler: impl Handler) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                println!("Parsing request...");
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e)
                };
                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }
            Err(e) => println!("Failed to read from connection: {}", e)
        }
            
    }
}