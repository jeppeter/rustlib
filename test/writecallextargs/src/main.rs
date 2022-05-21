use tempfile::TempDir;
use std::env;
use std::thread;
use std::time;
use std::fs;

#[derive(Debug)]
struct ExtArgsDir {
	srcdir : String,
	workdir :String,
	gendir :String,	
	reserved : bool,
	tdir : TempDir,
}

impl ExtArgsDir {
	pub fn new(workdir :&str,gendir :&str) -> ExtArgsDir {
		let retv :ExtArgsDir = ExtArgsDir{
			srcdir : "".to_string(),
			workdir : format!("{}",workdir),
			gendir : format!("{}",gendir ),
			reserved : false,
			tdir : TempDir::new().unwrap(),
		};
		retv
	}

}

impl Drop for ExtArgsDir {
	fn drop(&mut self) {
		if self.srcdir.len() > 0 && !self.reserved {
			
		}
		self.srcdir = "".to_string();
	}
}


fn main() {
    let args :Vec<String> = env::args().collect();
    if args.len() >= 3 {
    	let d :ExtArgsDir = ExtArgsDir::new(&(args[1]),&(args[2]));
    	println!("d  [{:?}]", d);
    }
}
