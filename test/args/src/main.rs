use std::env;
use std::i64;
use std::fmt::Write;

fn main() {
	let mut i=0;
	let mut v :i64;
	let mut base :u32;
	let mut cparse :String;
	let mut wstr :String = String::new();

    for arg in env::args() {
    	writeln!(wstr,"[{}]=[{}]", i, arg).unwrap();
    	i += 1;
    	if i == 1 {
    		continue;
    	}
    	base = 10;
    	cparse = format!("{}",arg);
    	if arg.starts_with("0x") || arg.starts_with("0X") {
    		cparse = cparse[2..].to_string();
    		base = 16;
    	}
    	match i64::from_str_radix(&cparse,base) {
    		Ok(c) => {
    			v=  c;
		    	//println!("[{}]={}",arg,v);
		    	writeln!(wstr,"[{}]={}",arg,v).unwrap();

    		},
    		Err(e) => {
    			//println!("[{}] error[{:?}]", arg,e);
    			writeln!(wstr,"[{}] error[{:?}]", arg,e).unwrap();
    		}
    	}
    }

    print!("total str\n{}",wstr);
}
