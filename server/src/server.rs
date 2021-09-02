use std::net::TcpListener;

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
                Ok((stream, _)) => {
                },
                Err(e) => println!("Failed to establish a connection {}", e)
                
            }
        }
    }
}