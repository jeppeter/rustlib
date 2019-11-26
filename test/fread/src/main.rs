//use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};


fn  read_file(path :&str) -> Result<String,std::io::Error>  {
	let fd ;
	let mut cstr :String = String::from("");

	match File::open(path) {
		Ok(file) => fd  = file,
		Err(err)  => {println!("get error {} {:?}",path, err);return Err(err);}
	}
	let reader = BufReader::new(fd);
	for (i,l) in reader.lines().enumerate() {
		let l = l.unwrap();
		println!("[{}]=[{}]", i , l);
		cstr.push_str(&(format!("{}\n", l)[..]));
	}
	return Ok(cstr);
}

fn main() {
    let argv = std::env::args();
    let mut i = 0;
    for c in argv {
    	if i > 0 {
	    	//println!("{}", c);
	    	match read_file(&(c[..])) {
	    		Err(e) => {eprintln!("read {} error[{:?}]",c, e);},
	    		Ok(s) => {println!("read {}\n", c);;print!("{}", s);}
	    	}
    	}
    	i  +=  1;
    }
}
