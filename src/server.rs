use crate::http::{response, ParseError, Request, Response, SatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

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
                    println!("{} connected.", address);
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let res: Result<Request, ParseError> = Request::try_from(&buffer[..]);
                            let response = match res {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        SatusCode::Ok,
                                        Some("<h1>Hurray!!!</h1>".to_string()),
                                    )

                                    // Implement it in Response.send to avoid to copy the entire Response
                                    // on the heap.
                                    // write!(stream, "{}", response);
                                }
                                Err(e) => {
                                    println!("failed to parse a request: {}", e);
                                    Response::new(SatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {}", e);
                            }
                            // let res: Result<Request, String> = &buffer[..].try_into();
                        }
                        Err(e) => {
                            println!("failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("connection error: {}", e);
                    continue;
                }
            };
        }
    }
}
