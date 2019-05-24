use std::net::{TcpListener,TcpStream};
use std::thread;
use std::process;
use std::io::{Write,Read};
use std::env;
use std::vec::Vec;
//use std::fmt;

fn handle_client(mut stream: TcpStream) {
	loop {
		let mut readbuf = [0;1600];
		match stream.read(&mut readbuf) {
			Ok(n) => {
				if n == 0 {
					break;
				}
				stream.write(&readbuf[0..n]).unwrap();
			},
			Err(err) => {
				panic!(err);
			}
		}
	}
}

fn main() {
	let argv :Vec<String> = env::args().collect();
	let mut port:i32 = 3102;
	let bindstr :String;
	if argv.len() > 1 && 
		(argv[1] == "-h" ||
			argv[1] == "--help") {
			println!("{} port", argv[0]);
			process::exit(0);
		}

	if argv.len() > 1 {
		port = argv[1].parse().unwrap_or(3102);
	}
	bindstr = format!("127.0.0.1:{}", port);
	let listener = TcpListener::bind(bindstr).unwrap();
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move || {
					handle_client(stream)
				});
			},
			Err(err) => {
				println!("can not accept {} error [{}]", bindstr, err);
			}
		}
	}
	return;
}