use std::fs::File;
use std::io::{Write};

fn write_file(path :String) {
	let mut fd;
	let mut _spath = path.clone();

	match File::create(path) {
		Ok(file) => fd = file,
		Err(e) => {println!("can not open {} for write [{:?}]", _spath,e); return;}
	}
	match fd.write(b"hello world") {
		Ok(retn) => {println!("write {} succ [{:?}]", _spath,retn);return;},
		Err(e) => {println!("write {} error[{:?}]", _spath,e);return}
	}
	return;
}


fn main() {
	let argv = std::env::args();
	let mut i = 0;
	for c in argv {
		if i > 0 {
			write_file(c);
		}
		i += 1;
	}
}
