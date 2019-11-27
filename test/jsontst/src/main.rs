use serde_json::{Value,json};
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

fn format_tabs(tabs :i32) -> String {
	let mut retstr:String = String::from("");
	for _ in 0..tabs {
		retstr.push_str(&(format!("    ")[..]));
	}
	return retstr;
}


fn array_json(_key :&str, _instr :&str, tabs :i32) -> String {
	let mut retstr :String = String::from("");
	let vv :&Vec<Value>;
	let d :Value;
	//let mut cnt:usize;
	match serde_json::from_str(_instr) {
		Ok(v)  => {
			d = v;
			match d.as_array() {
				Some(dv) => { vv = dv;},
				None => {eprintln!("---------------\n{}+++++++++++++++++\nnot array",_instr); return retstr;}
			}
		}, 
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	retstr.push_str(&(format_tabs(tabs)[..]));
	if _key.len() > 0 {
		retstr.push_str(&(format!("\"{}\" : [",_key)[..]));
	} else {
		retstr.push_str("[");
	}

	for (i,v) in vv.iter().enumerate() {
		if i > 0 {
			retstr.push_str(",");
		}
		retstr.push_str("\n");
		if v.is_object() {
			retstr.push_str(&(enumerate_json("",&(v.to_string()[..]), tabs + 1)[..]));
			continue;
		}
		if v.is_array() {
			retstr.push_str(&(array_json("",&(v.to_string()[..]),tabs + 1)[..]));
			continue;
		}

		retstr.push_str(&(format_tabs(tabs + 1)[..]));
		retstr.push_str(&(format!("{}",v.to_string())[..]));
	}

	retstr.push_str("\n");
	retstr.push_str(&(format_tabs(tabs)[..]));
	retstr.push_str(&(format!("]")[..]));
	return retstr;
}

fn enumerate_json(_key :&str,_instr :&str,tabs :i32) -> String {
	let d :HashMap<String,Value>;
	let mut retstr :String = String::from("");
	let mut i:i32;
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	retstr.push_str(&(format_tabs(tabs)[..]));
	if _key.len() > 0 {
		retstr.push_str(&(format!("\"{}\" : {{",_key)[..]));		
	} else {
		retstr.push_str(&(format!("{{")[..]));
	}


	i = 0;
	for (s,v) in d {
		if i > 0 {
			retstr.push_str(",");
		}
		retstr.push_str("\n");
		if v.is_object() {
			retstr.push_str(&(enumerate_json(&(s[..]),&(v.to_string()[..]),tabs + 1)[..]));
			i += 1;
			continue;
		}

		if v.is_array() {
			retstr.push_str(&(array_json(&(s[..]),&(v.to_string()[..]),tabs+1)[..]));
			i += 1;
			continue;
		}

		retstr.push_str(&(format_tabs(tabs+1)[..]));
		retstr.push_str(&(format!("\"{}\" : {}",s,v)[..]));
		i += 1;
	}
	if i > 0 {
		retstr.push_str("\n");
	}
	retstr.push_str(&(format_tabs(tabs)[..]));
	retstr.push_str(&(format!("}}")[..]));
	return retstr;
}

fn add_json_value(_instr :&str, _key :&str, _val:&str) -> String {
	let mut d :HashMap<String,Value>;
	let c :Value;
	let mut retstr :String = String::from("");
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	match serde_json::from_str(_val) {
		Ok(v) => {c = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}

	d.insert(String::from(_key),c);
	retstr = enumerate_json("",&(format!("{}",json!(d).to_string())[..]),0);
	return retstr;
}

fn del_json_value(_instr :&str, _key :&str) -> String {
	let mut d :HashMap<String,Value>;
	let mut retstr :String = String::from("");
	match serde_json::from_str(_instr) {
		Ok(v) => {d = v;},
		Err(e) => {eprintln!("---------------\n{}+++++++++++++++++\nparse error [{:?}]",_instr,e); return retstr;}
	}
	if d.contains_key(_key) {
		d.remove(_key);
	}
	retstr = enumerate_json("",&(format!("{}",json!(d).to_string())[..]),0);
	return retstr;
}


fn usage(ec :i32,_fmtstr :String) {
	let mut outstr :String = String::from("");
	if _fmtstr.len() > 0 {
		outstr.push_str(&(_fmtstr[..]));
		outstr.push_str(&(format!("\n")[..]));
	}
	outstr.push_str(&(format!("josntst [SUBCOMMANDS]\n[SUBCOMMANDS]\n")[..]));
	outstr.push_str(&(format!("\tenumerate file...                   to enumerate file\n")[..]));
	outstr.push_str(&(format!("\tadd file key value                  to add the key value\n")[..]));
	outstr.push_str(&(format!("\tdel file keys...                    to delete key\n")[..]));
	if ec == 0 {
		print!("{}",outstr);
	} else {
		eprint!("{}",outstr);
	}

	std::process::exit(ec);
}


fn main() {
	let argv :Vec<String> = std::env::args().collect();
	let mut i:usize;
	let mut ss:String;
	let mut retstr :String;

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
					println!("{}",enumerate_json("",&(s[..]),0));
				},
				Err(e) => {
					eprintln!("can not read {} [{:?}]", argv[i],e );
				}
			}
			i += 1;
		}
	} else if argv[1] == "add" {
		if argv.len() < 5 {
			usage(3,format!("add need 5 args"));
		}
		match read_file(&(argv[2][..])) {
			Ok(s) => {
				ss = s;
				println!("add [{}] [{}] into\n{}\n----------------\n{}",
					argv[3],argv[4],ss, add_json_value(&(ss[..]),&(argv[3][..]),&(argv[4][..])));
			},
			Err(e) => {
				eprintln!("can not read {} [{:?}]", argv[2],e );
			}
		}
	} else if argv[1] == "del" {
		if argv.len() < 4 {
			usage(3,format!("del need 4 args"));
		}

		match read_file(&(argv[2][..])) {
			Ok(s) => {
				ss = s;
				i = 3;
				while i < argv.len() {
					retstr = del_json_value(&(ss[..]), &(argv[i][..]));
					println!("delete {}\n{}\n----------------\n{}\n++++++++++++++++++", argv[i], ss, retstr);
					ss = retstr;
					i +=1;
				}
			},
			Err(e) => {
				eprintln!("can not read {} [{:?}]", argv[2],e );
			}
		}
	}else {
		usage(3,format!("not support {}", argv[1]));
	}
	return
	    
}
