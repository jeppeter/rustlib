use regex::Regex;

fn capture_regex(restr :&str, instr :&str) -> bool {
	let re;
	match Regex::new(restr) {
		Err(e) => {println!("{} not compiled {:?}", restr,e);;return false;}
		Ok(v) => {re = v;}
	}
	let caps = re.captures(instr);
	match caps {
		Some(v) => {println!("capture {:?}", v); return true;}
		None => {println!("error {} {:?}", restr,instr); return false;}
	}
}

fn match_regex(restr :&str, instr :&str) -> bool {
	let re ;
	match Regex::new(restr) {
		Err(e) => {println!("{} not compiled {:?}",restr,e ); return false;}
		Ok(v) => {re = v;}
	}

	let bmatch = re.is_match(instr);
	if bmatch {
		println!("{} match {}", instr, restr);
		return true;
	}
	println!("{} not match {}", instr, restr );
	return false;
}

fn usage(ec :i32,_fmtstr :String) {
	let mut outstr :String = String::from("");
	if _fmtstr.len() > 0 {
		outstr.push_str(&(_fmtstr[..]));
	}
	outstr.push_str(&(format!("retest [SUBCOMMANDS]\n[SUBCOMMANDS]\n")[..]));
	outstr.push_str(&(format!("\tcapture restr instr...              to find all matches\n")[..]));
	outstr.push_str(&(format!("\tmatch   restr instr...              to match string\n")[..]));
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

	if argv[1] == "capture" {
		if argv.len() < 4 {
			usage(3,format!("capture need 4 args"));
		}
		i = 3;
		while i < argv.len() {
			capture_regex(&(argv[2][..]),&(argv[i][..]));
			i = i + 1;
		}
	} else if argv[1] == "match"{
		if argv.len() < 4 {
			usage(3, format!("match need 4 args"));
		}
		i = 3;
		while i < argv.len() {
			match_regex(&(argv[2][..]), &(argv[i][..]));
			i = i + 1;
		}
	}else {
		usage(3,format!("not support {}", argv[1]));
	}
	return
}