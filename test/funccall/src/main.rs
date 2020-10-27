use std::collections::HashMap;
use std::any::Any;

fn call_func_1(s :&str) {
	println!("{} call 1",s);
	return;
}

fn call_func_2(s :&str) {
	println!("{} call 2", s);
	return;
}



fn main() {
	type FuncString = fn(&str);
	let mut dispatch :HashMap<_,&dyn Any> = HashMap::new();
	let mut func :&dyn Any;
	let s :&dyn Any;
	let funcall :FuncString;
	func = &call_func_1;
	println!("{:?}", func);
	//println!("{}", &call_func_1);
	dispatch.insert("call_func_1",func);
	func = &call_func_2;
	dispatch.insert("call_func_2",func);
	println!("{:?}",dispatch["call_func_1"]);
	s = dispatch["call_func_1"];
	match s.downcast::<FuncString>() {
		Some(c) => {println!("down succ");},
		_ => {eprintln!("not succ");}
	}
	//funcall("cc");
	//dispatch["call_func_1"]("cc");
	//dispatch["call_func_2"]("cc");
	return;
}
