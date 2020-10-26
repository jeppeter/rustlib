use std::collections::HashMap;

fn call_func_1(s :&str) {
	println!("{} call 1",s);
	return;
}

fn call_func_2(s :&str) {
	println!("{} call 2", s);
	return;
}

fn main() {
	let mut dispatch :HashMap<_,fn(&str)> = HashMap::new();
	dispatch.insert("call_func_1",call_func_1);
	dispatch.insert("call_func_2",call_func_2);
	dispatch["call_func_1"]("cc");
	dispatch["call_func_2"]("cc");
	return;
}
