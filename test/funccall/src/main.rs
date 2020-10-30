use std::collections::HashMap;
//use std::any::Any;

fn call_func_1(s :&str) {
	println!("{} call 1",s);
	return;
}

fn call_func_2(s :&str) {
	println!("{} call 2", s);
	return;
}

fn call_func_base_1() {
	println!("base 111");
	return;
}

type FuncString = fn(&str);
type FuncBase = fn();


pub enum Functype {
	FuncS(FuncString),
	FuncB(FuncBase),
}

fn main() {
	let mut dispatch :HashMap<_,Functype> = HashMap::new();
	//let mut funcall :FuncString;
	//println!("{:?}", func);
	//println!("{}", &call_func_1);
	dispatch.insert("call_func_1",Functype::FuncS(call_func_1));
	dispatch.insert("call_func_2",Functype::FuncS(call_func_2));
	dispatch.insert("call_func_base_1",Functype::FuncB(call_func_base_1));
	//println!("{:?}",dispatch["call_func_1"]);
	match dispatch["call_func_1"] {
		Functype::FuncS(func) => {func("cc") ;println!("down succ");},
		_ => {eprintln!("not succ");}
	}

	match dispatch["call_func_base_1"] {
		Functype::FuncB(func) => {func();println!("base call");}
		_ => {eprintln!("no base");}
	}

	match dispatch["call_func_2"] {
		Functype::FuncB(func) => {func();println!("call base");}
		_ => {eprintln!("no func 2");}
	}
	//funcall("cc");
	//dispatch["call_func_1"]("cc");
	//dispatch["call_func_2"]("cc");
	return;
}
