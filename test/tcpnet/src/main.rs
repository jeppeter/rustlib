
//use std::sync::{Arc};
//use std::thread;
//use std::net::{TcpStream,TcpListener,Shutdown,SocketAddr};
use std::net::{TcpStream,TcpListener,Shutdown};
use std::io::{Read,Write};
use std::thread;

fn handle_client(mut stream: TcpStream) -> bool{
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    loop {
		match stream.read(&mut data) {
		        Ok(size) => {
		            // echo everything!
		            match stream.write(&data[0..size]) {
		            	Err(e) => {
		            		eprintln!("write error {:?}",e);
		            		stream.shutdown(Shutdown::Both).unwrap();
		            		return false;
		            	},
		            	Ok(_) => {return true;}
		            }
		        },
		        Err(_) => {
		            println!("An error occurred, terminating");
		            stream.shutdown(Shutdown::Both).unwrap();
		            return false;
		        }
		    }    	
    }
}


fn server_handler(port :String) -> i32 {
	let bindstr :String;
	let ebindstr :String;
	let server :TcpListener;
	bindstr = format!("0.0.0.0:{}", port);
	ebindstr = bindstr.clone();

	match TcpListener::bind(bindstr) {
		Ok(t) => {server = t;},
		Err(e) => {
			eprintln!("bind [{}] error {:?}", ebindstr,e);
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
				thread::spawn(move || {
					handle_client(client);
				});
				//handle_client(client);
				/*match client.shutdown(Shutdown::Both) {
					Ok(()) => {println!("shutdown {:?}", _sock);},
					Err(e) => {eprintln!("shutdown error {:?}", e);}
				}*/
			},
			Err(e) => {
				eprintln!("get error {:?}",e);
			}
		}
	}
	//return 0;
}

fn client_handler(host :String,port :String) -> i32 {
	let connstr :String;
	let v :String;
	connstr = format!("{}:{}",host,port);
	v = connstr.clone();
	match TcpStream::connect(connstr) {
		Ok(mut stream) => {
			let mut lines :String = String::with_capacity(512);
			match stream.write(b"hello"){
				Ok(n) => {println!("n [{}]",n);},
				Err(e) => {
					eprintln!("write [{}] error[{}]", v,e);
					return -5;
				},
			}
			match stream.read_to_string(&mut lines) {
				Ok(n) => {println!("n [{}]",n );},
				Err(e) => {
					eprintln!("read [{}] [{}]", v,e);
					return -6;
				},
			}
			return 0;
		},
		Err(e) => {
			eprintln!("error [{}] [{}]", v, e);
			return -3;
		}
	}
}

fn main() {
	let args :Vec<String> = std::env::args().collect();
	let port :String;
	let host :String;
	let ret :i32;
	if args.len() < 3 {
		eprintln!("{} command", args[0]);
		eprintln!("server port");
		eprintln!("client host port");
		std::process::exit(3);
	}


	if args[1] == "server" {
		port = args[2].clone();
		ret = server_handler(port);
	}  else if args[1] == "client" {
		host = args[2].clone();
		port = args[3].clone();
		ret = client_handler(host,port);
	} else {
		ret = -3;
	}
	std::process::exit(ret);	
}
