
use std::sync::{Arc};
use std::thread;
use std::net::{TcpStream,TcpListener,Shutdown};
use std::io::{Read,Write};


fn main() {
	let args :Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("{} command", args[0]);
		eprintln!("server port");
		eprintln!("client host port");
		std::process::exit(3);
	}
}
