use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let res: Result<Request, String> = Request::try_from(&buffer[..]);
                            match res {
                                Ok(request) => {},
                                Err(e) => println!("failed to parse a request: {}", e),
                            }
                            // let res: Result<Request, String> = &buffer[..].try_into();
                        },
                        Err(e) => {
                            println!("failed to read from connection: {}", e);
                        },
                    }
                },
                Err(e) => {
                    println!("connection error: {}", e);
                    continue;
                },
            };
        }
    }
}
