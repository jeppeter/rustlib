
use serde_json::Value;
mod bob;
use bob::{ExtKeyParse};

fn main() {
	let kev = bob::newkey();
	let _c  :ExtKeyParse = kev.clone();
	let nopt = bob::newoptions();
	let _d = nopt.clone();

	nopt.borrow_mut().insert("he",Value::Null);
	nopt.borrow_mut().insert("bb",Value::Null);
	_c.borrow_mut().set_string("hecc","vvvv");
	_c.borrow_mut().set_string("hecc2","vvvv");

	println!("nopt\n{}\n_d\n{}",nopt.borrow().string(), _d.borrow().string());
	println!("kev\n{}\n_c\n{}",kev.borrow().string(),_c.borrow().string());
}
