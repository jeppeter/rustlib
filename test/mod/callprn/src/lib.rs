
mod server;
mod client;

pub fn connect() -> bool {
	server::connect();
	client::connect();
	println!("connect function");
	return true;
}

#[cfg(test)]
mod lib_test {
	use super::*;

	#[test]
	fn connect_test() {
		assert_eq!(true, connect());
	}

	#[test]
	fn connect_client_test() {
		assert_eq!(true, client::connect());
	}

	#[test]
	fn connect_server_test() {
		assert_eq!(true, server::connect());
	}

}