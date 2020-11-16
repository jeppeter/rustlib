use std::io;

fn main() {
    let mut line :String;
    let mut newline :String;
    let mut tobytes :&[u8];
    let mut lastn :i32;
    let mut curn :i32;
    line = String::new();
    lastn = 0;
    loop {

    	match io::stdin().read_line(&mut line) {
    		Ok(n) => {
    			curn = n as i32;
    			tobytes = line.as_bytes();
    			newline = String::from(tobytes[lastn..curn]);
    			println!("{:?}", newline);},
    			lastn = curn;
    		Err(e) => {eprintln!("{:?}", e);}
    	}
    }
}
