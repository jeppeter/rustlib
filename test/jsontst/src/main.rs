use serde_json::{Value};
use std::fs::File;
use std::io::{BufRead,BufReader,Error};
use std::collections::HashMap;

fn  read_file(path :&str) -> Result<String,Error>  {
	let fd ;
	let mut cstr :String = String::from("");

	match File::open(path) {
		Ok(file) => fd  = file,
		Err(err)  => {println!("get error {} {:?}",path, err);return Err(err);}
	}
	let reader = BufReader::new(fd);
	for (_,l) in reader.lines().enumerate() {
		let l = l.unwrap();
		cstr.push_str(&(format!("{}\n", l)[..]));
	}
	return Ok(cstr);
}




fn enumerate_json(_key :&str,_instr :&str,tabs :i32) {
	let d :HashMap<String,Value>;
	let mut curstr :String;
	let mut i:i32;
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return;}
	}
	if _key.len() > 0 {
		curstr = String::from("");
		for _ in 0..tabs {
			curstr.push_str("    ");
		}

		curstr = format!("\"{}\" : \\{",_key);
	} else {
		curstr = format!("{");
	}
	println!("{}",curstr);

	for (s,v) in d {
		if v.is_object() {
			enumerate_json(s,v.to_string(),tabs + 1);
			continue;
		}
		curstr = String::from("");
		for _ in 0..tabs {
			curstr.push_str("    ");
		}
		curstr.push_str("\"{}\" : {}",s,v);
		println!("{}", curstr);
	}	
	return;
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
		i = 2;
		while i < argv.len() {
			match read_file(&(argv[i][..])) {
				Ok(s) => {
					enumerate_json(&(s[..]),0);
				},
				Err(e) => {
					eprintln!("can not read {} [{:?}]", argv[i],e );
				}
			}
			i += 1;
		}
	} else {
		usage(3,format!("not support {}", argv[1]));
	}
	return
	    
}
