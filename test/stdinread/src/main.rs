use std::io;

fn main() {
    let mut line :String;
    line = String::new();
    loop {

    	match io::stdin().read_line(&mut line) {
    		Ok(n) => {
    			print!("{}", line);
    			line = String::new();
    		},
    		Err(e) => {eprintln!("{:?}", e);}
    	}
    }
}
