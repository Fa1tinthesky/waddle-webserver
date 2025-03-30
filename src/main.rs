// use threadpool::ThreadPool;
use WSCC::{ router::{Router}, ThreadPool };
use std::{ 
    fs, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}, process, thread::{self, sleep}, time::Duration
};


fn main() {
    let aux_addrs = generate_aux_ports(8080);
    let fk_addrs = vec![SocketAddr::from(([127, 0, 0 ,1], 8070))]; // For testing purposes.

    let listener = start_ws(aux_addrs).unwrap_or_else(|| {
        println!("FAILED TO START A SERVER. TIMEOUT ERROR");
        process::exit(1);
    });

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

    } 
    
    println!("Shutting down.");
}


fn start_ws(addrs: Vec<SocketAddr>) -> Option<TcpListener> {
    let max_attempts = 5;
    let mut attempts = 0;

    let listener = loop {
        match TcpListener::bind(&addrs[..]) {
            Ok(listener) => { 
                println!("Server is running on: {:?}", listener.local_addr().unwrap());
                break Some(listener);
            },
            Err(err) => {
                attempts += 1;
                if attempts > max_attempts {
                    return None;
                }

                println!("{}/{} COULDN'T START A SERVER. RETRYING...", attempts, max_attempts);
                sleep(Duration::new(5, 0));
            }
        }
    };

    return listener;
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // match request_line
    let request_line = buf_reader.lines().next().expect("Problem is here, at line 59").unwrap(); 
    // buf_reader.lines().next().unwrap()..

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./html/index.html"),
        "GET /sleep HTTP/1.1" => {
            println!("Request to GET /sleep HTTP/1.1 was made");
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "./html/index.html")
        },
        "GET /index.js HTTP/1.1" => {
            let router = Router::new();
            match router.get_static_file("./js/index.js") {
                Some(x) => println!("FILE FOUND AT: {}", x),
                None => println!("FILE NOT FOUND 404"),
            }
    
            ("HTTP/1.1 200 OK", "./js/index.js")
        } 
        _ => ("HTTP/1.1 404 NOT FOUND", "./html/404.html")
    };


    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let http_response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(http_response.as_bytes()).unwrap();
}

fn generate_aux_ports(base_port: u16) -> Vec<SocketAddr> { 
    let mut addrs = vec![];
    for i in base_port-10..=base_port+10 {
        // addrs.push(SocketAddr::from(([127, 0, 0, 1], i)));
        addrs.push(SocketAddr::from(([0, 0, 0, 0], i)));
    }

    return addrs;
}
