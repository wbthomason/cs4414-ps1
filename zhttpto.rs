//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::{str,os};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    static mut number_visitors : int = 0;
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
                        unsafe { number_visitors += 1; }
                        let lineOne = match request_str.line_iter().nth(0) {
                                        Some(data) => data,
                                        None => ""
                                    };
                        let getIndex = lineOne.find_str("GET");
                        let httpIndex = lineOne.find_str("HTTP/1.1");
                        let response: ~str = unsafe { 
                            match (getIndex, httpIndex) {
                                (Some(i), Some(j)) if (j - i) > 6 => loadFile(lineOne.slice(i+5,j-1)),
                                (_,_) => fmt!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             <h2> Congratulations! You are visitor number %d.<h2>
                             </body></html>\r\n", number_visitors)
                            }
                        };
                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}

fn loadFile(path : &str) -> ~str {
    let file = os::make_absolute(&Path(path));
    let return_string : ~str = fmt!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>%s</title></head>
                             <body>
                             %s
                             </body></html>\r\n", path, 
                                if os::path_exists(&file) && ! os::path_is_dir(&file) {
                                    match std::io::read_whole_file_str(&file) {
                                        Ok(data) => {
                                            data
                                        }
                                        Err(error) => {
                                            println(error);
                                            error
                                        }
                                    }
                                }
                                else {
                                    ~"Invalid path or path is directory."
                                });
    return_string
}