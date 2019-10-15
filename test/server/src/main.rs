use std::net::TcpListener;


fn main() {
	let bindstr :String;
	let argv: Vec<String> = std::env::args().collect();
	let listener ;

	if argv.len() < 2 {
		println!("{} host:port", argv[0]);
		std::process::exit(3);
	}
	bindstr = argv[1].clone();

	match TcpListener::bind(bindstr) {
		Ok(t) => listener = t,
		Err(e) => {let c = bindstr;println!("can not bind {} error{:?}",c, e ); std::process::exit(5);}
	}

	loop {
		let conn;
		match listener.accept() {
			Ok(a) => {conn = a;},
			Err(e) => {println!("accept error{:?}", e);}
		}
	}

}
