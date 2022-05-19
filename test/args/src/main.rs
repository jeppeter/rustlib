use std::env;
use std::i64;
use std::io::{Write,BufWriter};

fn write_fmts<T : Write>(iowrite :&mut T,s :&str) -> Result<usize,std::io::Error> {
	let mut c = iowrite.write(s.as_bytes())?;
	c += iowrite.write(b"\n")?;
	Ok(c)
	//iowrite.write_str("\n")
}

fn main() {
	let mut i=0;
	let mut v :i64;
	let mut base :u32;
	let mut cparse :String;
	//let mut wstr :String = String::new();
	let mut buf = vec![];
	{
		let mut wstr = BufWriter::new(&mut buf);
		//let mut wstr = std::io::stdout();

		let mut cstr :String;

	    for arg in env::args() {
	    	cstr = format!("[{}]=[{}]", i, arg);
	    	write_fmts(&mut wstr,&cstr).unwrap();
	    	//writeln!(wstr,"[{}]=[{}]", i, arg).unwrap();
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
			    	cstr = format!("[{}]={}",arg,v);
			    	write_fmts(&mut wstr, &cstr).unwrap();
			    	//writeln!(wstr,"[{}]={}",arg,v).unwrap();

	    		},
	    		Err(e) => {
	    			//println!("[{}] error[{:?}]", arg,e);
	    			cstr = format!("[{}] error[{:?}]", arg,e);
	    			write_fmts(&mut wstr,&cstr).unwrap();
	    		}
	    	}
	    }
	}
    let s = std::str::from_utf8(&buf).unwrap();
    print!("total str\n{}",s);
}
