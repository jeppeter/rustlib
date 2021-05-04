use std::error::Error;
use std::boxed::Box;
use std::fmt;

/*
#[derive(Debug,Clone)]
struct CError
{
	msg :String,
}

impl CError {
	fn new(c :&str) -> CError {
		CError{ msg : format!("{}", c),}
	}
}

impl fmt::Display for CError {
	fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.msg)
	}
}



impl Error for CError {}
*/

macro_rules! ErrorCreate {
	($type:ident) => {
	#[derive(Debug,Clone)]
	struct $type {
		msg :String,		
	}

	impl $type {
		fn new(c :&str) -> $type {
			$type {msg : format!("{}",c)}
		}
	}

	impl fmt::Display for $type {
		fn fmt(&self,f :&mut fmt::Formatter) -> fmt::Result {
			write!(f,"{}",self.msg)
		}
	}

	impl Error for $type {}
	};
}


ErrorCreate!{BaseError}
ErrorCreate!{CError}

macro_rules! NewError {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a)*)[..]));
		Err(Box::new(<$type>::new(c.as_str())))
	  }
	};
}

fn call_1() -> Result<i32,Box<dyn Error>> {
	//Err(Box::new(CError::new(&(format!("[{}:{}]call_1 error",file!(),line!())[..]))))
	//
	NewError!(CError,"call_1 error")
}

fn call_2() -> Result<i32,Box<dyn Error>> {
	let c = call_1()?;
	Ok(c)
}

fn call_3() -> Result<i32,Box<dyn Error>> {
	let c = call_2()?;
	Ok(c)
}

fn base_1() -> Result<i32,Box<dyn Error>> {
	//Err(Box::new(CError::new(&(format!("[{}:{}]call_1 error",file!(),line!())[..]))))
	//
	NewError!(BaseError,"base_1 error")
}

fn base_2() -> Result<i32,Box<dyn Error>> {
	let c = base_1()?;
	Ok(c)
}

fn base_3() -> Result<i32,Box<dyn Error>> {
	let c = base_2()?;
	Ok(c)
}

fn main() {
	match call_3() {
		Ok(c) => println!("i {}", c),
		Err(e)  => eprintln!("{}", e),
	}

	match base_3() {
		Ok(c) => println!("i {}", c),
		Err(e)  => eprintln!("{}", e),
	}
}
