use std::{
     io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("connection made");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                let stream = stream.unwrap();
                handle_connection(stream)
            },
            Err(_) => {
                println!(" error during connection ");
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream) {
    
    let mut buffer = [0; 2048];
    let stream_data_size = stream.read(&mut buffer).unwrap();

    let buffer = vec![0; stream_data_size];

    let mut headers = [httparse::EMPTY_HEADER; 4];
    let mut req = httparse::Request::new(&mut headers);
    println!("{buffer:?}");
    let res = req.parse(&buffer).unwrap();

    let buf_reader = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Headers: {:?}", res);
}