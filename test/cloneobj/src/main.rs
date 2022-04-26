
use serde_json::Value;
mod bob;
use bob::{ExtKeyParse};

fn main() {
	let kev = bob::ExtKeyParse::new();
	let _c  :ExtKeyParse = kev.clone();
	let nopt = bob::ExtArgsOptions::new();
	let _d = nopt.clone();

	nopt.insert("he",Value::Null);
	nopt.insert("bb",Value::Null);
	kev.set_string("hecc","vvvv");
	_c.set_string("hecc2","vvvv");

	println!("nopt\n{}\n_d\n{}",nopt.string(), _d.string());
	println!("kev\n{}\n_c\n{}",kev.string(),_c.string());
}
