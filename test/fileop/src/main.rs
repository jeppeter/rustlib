#[allow(unused_imports)]
use std::fs;
use std::env;
use std::vec::Vec;
use std::process;
use std::io;


fn usage(ec :i32, fmtstr :&str) {
	let mut outputs :String = String::from("");
	if fmtstr.len() > 0 {
		outputs.push_str(fmtstr);
	}
	outputs.push_str(&(format!("fileop [OPTIONS] [COMMANDS]\n")[..]));
	outputs.push_str(&(format!("[OPTIONS]\n")[..]));
	outputs.push_str(&(format!("    --help|-h              to display this help information\n")[..]));
	outputs.push_str(&(format!("[COMMANDS]\n")[..]));
	outputs.push_str(&(format!("    exist  [files]....     to check for files...\n")[..]));

	if ec != 0 {
		eprintln!("{}", outputs);
	}else {
		println!("{}",outputs);
	}
	process::exit(ec);
}

fn list_dir(dname :&str) -> Result<Vec<String>,io::Error> {
	let mut retv :Vec<String> = Vec::new();

	for entry in fs::read_dir(dname)? {
		let e = entry?;
		let path = e.path();
		if path.is_dir() {
			match path.to_str() {
				Some(s) => {
					let  cv :Vec<String> = list_dir(&s)?;
					let mut idx :usize  = 0;
					let mut ns :String;
					while idx < cv.len() {
						ns = String::from(&(cv[idx]));
						retv.push(ns);
						idx += 1;
					}
				},
				_ => {
					eprintln!("{:?} convert string error",path);
				}
			}
		} 
		match path.to_str() {
			Some(s) => {
				let ns = String::from(s);
				retv.push(ns);
			},
			_ => {
				eprintln!("{:?} convert string error",path);
			}
		}
		
	}

	Ok(retv)
}

fn main() {
	let argv :Vec<String> = env::args().collect();

	if argv.len() > 1 {
		if argv[1] == "-h" ||  argv[1] == "--help"{
			usage(0,"");
		} else if argv[1] == "listdir" {
			let mut idx :usize = 2;
			while idx < argv.len() {
				match list_dir(&(argv[idx])) {
					Ok(vs) => {
						let mut i :usize = 0;
						while i < vs.len() {
							println!("{}", vs[i]);
							i += 1;
						}
					},
					Err(e) => {
						eprintln!("can not list [{}] [{}]",argv[idx],e);
						process::exit(4);
					}
				}
				idx += 1;
			}
		}
	} 
	return;
}
