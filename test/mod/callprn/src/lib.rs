
mod server;
mod client;

pub fn connect() {
	server::connect();
	client::connect();
	println!("connect function");
}