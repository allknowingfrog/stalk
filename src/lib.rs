use std::collections::VecDeque;
use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next();

        let port = match args.next() {
            Some(arg) => arg,
            None => String::from("7878"),
        };

        let host = match args.next() {
            Some(arg) => arg,
            None => String::from("127.0.0.1"),
        };

        Config { port, host }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub struct Stalk {
    queue: VecDeque<String>,
}

impl Stalk {
    pub fn new() -> Stalk {
        let queue: VecDeque<String> = VecDeque::new();

        Stalk { queue }
    }

    pub fn run(&mut self, listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.process(stream);
        }
    }

    pub fn put(&mut self, data: &str) -> String {
        self.queue.push_back(String::from(data));
        String::from("CREATED\n")
    }

    pub fn get(&mut self) -> String {
        match self.queue.pop_front() {
            Some(job) => format!("{}\n{}", "FOUND", job),
            None => String::from("NONE"),
        }
    }

    pub fn process(&mut self, mut stream: TcpStream) {
        let request = Stalk::read_from_stream(&stream);

        let (command, data) = Stalk::parse_request(&request);

        let response = match command.as_ref() {
            "PUT" => self.put(data),
            "GET" => self.get(),
            _ => String::from("ERR\n"),
        };

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn read_from_stream(mut stream: &TcpStream) -> String {
        let mut buffer = [0; 8];
        let mut request = String::new();
        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    let part = String::from_utf8_lossy(&buffer[..size]);
                    request.push_str(&part);
                },
                Err(_) => {
                    break;
                }
            }
        }
        request
    }

    fn parse_request(request: &String) -> (&str, &str) {
        match request.find(" ") {
            Some(pos) => {
                (&request[..pos], &request[(pos+1)..])
            },
            None => {
                let pos = request.len();
                (&request[..], &request[pos..pos])
            },
        }
    }
}
