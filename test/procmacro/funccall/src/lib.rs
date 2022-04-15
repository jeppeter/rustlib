

type FnVoid = fn();

#[derive(Clone)]
pub struct FuncName {
	pub name :String,
	pub callfn :FnVoid,
}

impl FuncName {
	pub fn new(n :String,f :FnVoid) -> FuncName {
		FuncName {
			name: n,
			callfn: f,
		}
	}
}

pub fn call_functions(name :&str, fnvec :&Vec<FuncName>) {
	for v in  fnvec {
		if v.name == name {
			let c = v.callfn;
			c();
			return;
		}
	}
	println!("can not found [{}]", name);
	return;
}

