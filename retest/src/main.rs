use regex::Regex;

fn capture_regex(restr :&str, instr :&str) {
	let re = Regex::new(restr).unwrap();
	let caps = re.captures(instr);
	match caps {
		Some(v) => {println!("capture {:?}", v);}
		None => {println!("error {:?} {:?}", restr,instr);}
	}

}

fn usage(ec :i32,fmtstr :&str) {
	let outstr = format!("retest [SUBCOMMANDS]`n[SUBCOMMANDS]`n");
	if ec == 0 {
		print!("{:?}",outstr);
	} else {
		eprint!("{:?}",outstr);
	}

	std::process::exit(ec);
}

fn main() {
	let argv :Vec<String> = std::env::args().collect();

	println!("count {}", argv.len());
	if argv.len() < 2 {
		usage(3,"need at least 2 args");
	}
}