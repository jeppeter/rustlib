//use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};


fn  read_file(path :String)   {
	let fd ;
	let mut _spath = path.clone();

	match File::open(path) {
		Ok(file) => fd  = file,
		Err(err)  => {println!("get error {} {:?}",_spath, err);return;}
	}
	let reader = BufReader::new(fd);
	for (i,l) in reader.lines().enumerate() {
		let l = l.unwrap();
		println!("[{}]=[{}]", i , l);
	}
	return;
}

fn main() {
    let argv = std::env::args();
    let mut i = 0;
    for c in argv {
    	if i > 0 {
	    	println!("{}", c);
	    	read_file(c);    		
    	}
    	i  +=  1;
    }
}
