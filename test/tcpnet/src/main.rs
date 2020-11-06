
//use std::sync::{Arc};
//use std::thread;
//use std::net::{TcpStream,TcpListener,Shutdown,SocketAddr};
use std::net::{TcpStream,TcpListener,Shutdown};
//use std::io::{Read,Write};

fn server_handler(port :String) -> i32 {
	let bindstr :String;
	let server :TcpListener;
	bindstr = format!("0.0.0.0:{}", port);

	match TcpListener::bind(bindstr) {
		Ok(t) => {server = t;},
		Err(e) => {
			eprintln!("bind [{}] error {:?}", bindstr,e);
			return -3;
		}
	}

	loop {
		let client :TcpStream;
		//let sockaddr :SocketAddr;
		match server.accept() {
			Ok((cli,_sock)) => { 
				client=cli;
				//sockaddr = sock;
				client.shutdown(Shutdown::Both);
			},
			Err(e) => {
				eprintln!("get error {:?}",e);
			}
		}
	}
	//return 0;
}

fn main() {
	let args :Vec<String> = std::env::args().collect();
	if args.len() < 3 {
		eprintln!("{} command", args[0]);
		eprintln!("server port");
		eprintln!("client host port");
		std::process::exit(3);
	}


	if args[1] == "server" {
		server_handler(args[2]);
	} 
	
}
