
use serde_json::{Result,Value};

fn enumerate_json(instr :&str) {

}

fn read_file(fname :&str) -> std::Result<String,Error>{
	
}

fn usage(ec :i32,_fmtstr :String) {
	let mut outstr :String = String::from("");
	if _fmtstr.len() > 0 {
		outstr.push_str(&(_fmtstr[..]));
		outstr.push_str(&(format!("\n")[..]));
	}
	outstr.push_str(&(format!("josntst [SUBCOMMANDS]\n[SUBCOMMANDS]\n")[..]));
	outstr.push_str(&(format!("\tenumerate file...                   to enumerate file\n")[..]));
	if ec == 0 {
		print!("{}",outstr);
	} else {
		eprint!("{}",outstr);
	}

	std::process::exit(ec);
}


fn main() {
	let argv :Vec<String> = std::env::args().collect();
	let mut i;

	if argv.len() < 2 {
		usage(3,format!("need at least 2 args"));
	}

	if argv[1] == "enumerate" {
		if argv.len() < 3 {
			usage(3,format!("enumerate need 3 args"));
		}
		i = 3;
		while i < argv.len() {
			capture_regex(&(argv[2][..]),&(argv[i][..]));
			i = i + 1;
		}
	} else {
		usage(3,format!("not support {}", argv[1]));
	}
	return
	    
}
