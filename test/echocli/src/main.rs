use std::net::{TcpStream};
use std::io::{Write,Read};
use std::env;
//use std::process;
use std::vec::Vec;


fn main() {
	let argv :Vec<String> = env::args().collect();
	let mut port:i32 = 3102;
	let mut hoststr :String = String::from("127.0.0.1");
	let  bindstr:String;

	if argv.len() == 2 && 
		(argv[1] == "-h" || argv[1] == "--help") {
		println!("{} host port", argv[0]);
		return;
	}

	if argv.len() > 1 {
		hoststr = argv[1].clone();
	}
	
	if argv.len() > 2 {
		port = argv[2].parse().unwrap_or(3102);
	}
	
	bindstr = format!("{}:{}", hoststr ,port);

	match TcpStream::connect(bindstr) {
		Ok(mut stream) => {
			stream.write(b"hello");
			let mut data = [0 as u8 ;1000];
			match stream.read(&mut data) {
				Ok(_) => {
					println!("read data");
				}
				Err(e) => {
					println!("read error {}", e);
				}
			}
		}
		Err(e2) => {
			println!("connect error {}",  e2);
		}
	}

	return;    
}
