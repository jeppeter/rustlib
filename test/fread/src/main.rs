use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};


fn  read_file(path :String)   {
	let fd ;

	match File::open(path) {
		Ok(file) => fd  = file,
		Err(err)  => {return;}
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
    //let mut i = 0;
    for c in argv {
    	read_file(c);
    	//i  +=  1;
    }
}
