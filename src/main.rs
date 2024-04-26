use std::{
        error::Error, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, fs
};
use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("connection made");
    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                let stream = stream.unwrap();
                pool.execute( move||{   
                    let _ = handle_connection(stream);
                });
                pool.join();
            },
            Err(_) => {
                println!(" error during connection ");
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream) -> Result<usize,Box<dyn Error + 'static>> {
    let mut buf_reader = BufReader::new(&mut stream);

     let mut buffer = vec![];

    let _ = buf_reader.read(&mut buffer);
    
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let (request_type, path) = request_type(&http_request[0]).unwrap();
    println!("{:?}, {}",request_type, path);

    println!("Request: {:#?}", http_request);
    let contents = fs::read_to_string("src/a.html").unwrap();
    let length = contents.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    return Ok(1);
}
fn request_type(x: &str) -> Option<(String, String)> {
    let binding = x.to_string();
    let mut array_of_x = binding.split_whitespace();
    if let Some(first_word) = array_of_x.next() {
        match first_word {
            "GET" => return Some(("Get".to_string(), array_of_x.next().unwrap().to_string())),
            "POST" => return Some(("POST".to_string(), array_of_x.next().unwrap().to_string())),
            _ => return Some(("Something".to_owned(), "/".to_string()))
        }
    } else {
        println!("Nothing here!");
        return Some(("_".to_string(), "_".to_string()));
    }
}