use std::io::{self, Write};
use std::env;
use std::fs;

fn main() {
    let output = env::args().nth(1);
	let mut writer: Box<dyn Write> =  match output {
	    Some(x) => {
	        Box::new(fs::File::create(x).unwrap()) as Box<dyn Write>
	    },
    	None => Box::new(io::stdout()) as Box<dyn Write>,
	};
    match writer.write(b"new line") {
    	Ok(_) => {},
    	Err(_) => {}
    }

    match writer.flush() {
        Err(_e) => {
            eprintln!("can not flush");
        },
        _ => {}
    }

    loop {

    }
}
