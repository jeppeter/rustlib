use regex::Regex;

fn capture_regex(restr :&str, instr :&str) -> bool {
	let re;
	match Regex::new(restr) {
		Err(e) => {println!("{} not compiled {:?}", restr,e);return false;}
		Ok(v) => {re = v;}
	}
	let caps = re.captures(instr);
	match caps {
		Some(v) => {
			let mut i :usize;
			i = 0;
			println!("[{}] capture [{}]",restr,instr);
			while i < v.len() {
				println!("    [{}]=[{}]",i,v.get(i).map_or("", |m| m.as_str()));
				i += 1;
			}
			return true;
		},
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

fn split_regex(restr :&str, instr :&str) -> bool {
	let re;
	let mut i :usize;
	let sarr :Vec<&str>;
	let mut fmtstr :String;
	match Regex::new(restr) {
		Err(e) => {eprintln!("{} not compile {:?}",restr,e ); return false;}
		Ok(v) => {re = v;}
	}

	sarr = re.split(instr).into_iter().collect();
	i = 0;
	fmtstr = String::from("");
	fmtstr.push_str(&(format!("split [{}] with [{}] [",instr,restr)[..]));
	while i < sarr.len() {
		if sarr[i].len() > 0 {
			if i > 0 {
				fmtstr.push_str(&(format!(",")[..]));
			}
			fmtstr.push_str(&(format!("{}", sarr[i])[..]));			
		}
		i = i + 1;
	}
	fmtstr.push_str("]");

	println!("{}",fmtstr);
	return true;
}

fn replace_regex(restr :&str, instr :&str, replaced :&str,ball :bool) -> bool {
	let re;

	match Regex::new(restr) {
		Err(e) => {eprintln!("{} not compile {:?}",restr,e ); return false;}
		Ok(v) => {re = v;}
	}
	let v;
	if ball {
		v = re.replace_all(instr,replaced);
	} else {
		v = re.replace(instr,replaced);
	}
	println!("restr[{}] instr [{}] replaced[{}] [{:?}] => [{}]",restr,instr,replaced,ball,v);
	return true;
}

fn usage(ec :i32,_fmtstr :String) {
	let mut outstr :String = String::from("");
	if _fmtstr.len() > 0 {
		outstr.push_str(&(_fmtstr[..]));
	}
	outstr.push_str(&(format!("retest [SUBCOMMANDS]\n[SUBCOMMANDS]\n")[..]));
	outstr.push_str(&(format!("\tcapture restr instr...              to find all matches\n")[..]));
	outstr.push_str(&(format!("\tmatch   restr instr...              to match string\n")[..]));
	outstr.push_str(&(format!("\tsplit   restr instr...              to split string\n")[..]));
	outstr.push_str(&(format!("\treplace restr instr [replaced]      to replace string with restr => replaced \n")[..]));
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
	} else if argv[1] == "split" {
		if argv.len() < 4 {
			usage(3,format!("split need 4 args"));
		}
		i = 3;
		while i < argv.len() {
			split_regex(&(argv[2][..]),&(argv[i][..]));
			i = i + 1;
		}
	} else if argv[1] == "replace" {
		if argv.len() < 4 {
			usage(3,format!("replace need 3 args"));
		}
		let mut replaced :&str = "";
		let restr :&str = &argv[2];
		let instr :&str = &argv[3];
		if argv.len() >= 5 {
			replaced = &argv[4];
		}
		replace_regex(restr,instr,replaced,false);
	}  else if argv[1] == "replaceall" {
		if argv.len() < 4 {
			usage(3,format!("replace need 3 args"));
		}
		let mut replaced :&str = "";
		let restr :&str = &argv[2];
		let instr :&str = &argv[3];
		if argv.len() >= 5 {
			replaced = &argv[4];
		}
		replace_regex(restr,instr,replaced,true);
	} else {
		usage(3,format!("not support {}", argv[1]));
	}
	return
}