#[allow(unused_imports)]
use std::fs;
use std::env;
use std::vec::Vec;
use std::process;


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

fn main() {
	let argv :Vec<String> = env::args().collect();

	if argv.len() > 1 {
		if argv[1] == "-h" ||  argv[1] == "--help"{
			usage(0,"");
		}
	} 
	return;
}
