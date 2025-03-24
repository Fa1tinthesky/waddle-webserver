use std::{ 
    fs, 
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream}
};


fn main() {
    let aux_addrs = generate_aux_ports(8080);
    let listener = match TcpListener::bind(&aux_addrs[..]) {
        Ok(val) => { println!("The server is running on {:?}", val.local_addr().unwrap()); val },
        Err(err) => panic!("Error occured!!!")
    };

    for stream in listener.incoming() {
        println!("Buyaaaah, connection was made~!!!!");
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    println!("Hello, world!");
    
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|result| !result.is_empty())
        .collect();

    // println!("Request: {http_request:#?}");

    //let mut file = File::open("./html/index.html").unwrap();
    //let mut contents = String::new();
    //file.read_to_string(&mut contents).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./html/index.html").unwrap();
    let length = contents.len();

    let http_response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n");

    stream.write_all(http_response.as_bytes()).unwrap();
}

fn generate_aux_ports(base_port: u16) -> Vec<SocketAddr> { 
    let mut addrs = vec![];
    for i in base_port-10..=base_port+10 {
        addrs.push(SocketAddr::from(([127, 0, 0, 1], i)));
    }

    return addrs;
}
