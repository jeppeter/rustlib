use std::error::Error;



#[macro_export]
macro_rules! base_error_class {
	($type:ident) => {
		#[derive(Clone)]
		pub struct $type {
			msg :String,
			fname :String,
			lineno :u32,
			types :String,
		}

		#[allow(dead_code)]
		impl $type {
			fn create(fname :&str,lineno :u32, types :&str,c :&str) -> $type {
				$type {
					msg : format!("{}",c),
					fname : format!("{}",fname),
					lineno : lineno,
					types : format!("{}",types),
				}
			}
		}

		impl std::fmt::Display for $type {
			fn fmt(&self,f :&mut std::fmt::Formatter) -> std::fmt::Result {
				let mut errdisplay : bool =false;
				match std::env::var("ERROR_LEVEL") {
					Ok(vs) => {
						match vs.parse::<i32>() {
							Ok(v) => {
								if v >= 3 {
									errdisplay = true;
								}
							},
							Err(_e) => {
							}
						}
					},
					_ => {},
				}
				if errdisplay {
					write!(f,"[{}:{}][{}]{}",self.fname,self.lineno,self.types,self.msg)
				} else {
					write!(f,"{}",self.msg)	
				}
				
			}
		}

		impl std::fmt::Debug for $type {
			fn fmt(&self,f :&mut std::fmt::Formatter) -> std::fmt::Result {
				let mut errdisplay : bool =false;
				match std::env::var("ERROR_LEVEL") {
					Ok(vs) => {
						match vs.parse::<i32>() {
							Ok(v) => {
								if v >= 3 {
									errdisplay = true;
								}
							},
							Err(_e) => {
							}
						}
					},
					_ => {},
				}
				if errdisplay {
					write!(f,"[{}:{}][{}]{}",self.fname,self.lineno,self.types,self.msg)
				} else {
					write!(f,"{}",self.msg)	
				}				
			}
		}

		impl std::error::Error for $type {}
	};
}

#[macro_export]
macro_rules! base_new_error {
	($type:ty,$($a:expr),*) => {
		{
			let fname = format!("{}",file!());
			let lineno = line!();
			let types = format!("{}",stringify!($type));
			let mut c :String= format!("");
			c.push_str(&(format!($($a),*)[..]));
			return Err(Box::new(<$type>::create(&fname,lienno,&types,c.as_str())));
		}
	};
}


#[macro_export]
macro_rules! base_error_create {
	($type:ty,$($a:expr),*) => {
		{
			let fname = format!("{}",file!());
			let types = format!("{}",stringify!($type));
			let mut c :String= format!("");
			c.push_str(&(format!($($a),*)[..]));
			Box::new(<$type>::create(&fname,line!(),&types,c.as_str()))
		}
	};
}


base_error_class!{NBaseError}


fn call_error(ival :i32) -> Result<(),Box<dyn Error>> {
	let mut retval :Box<dyn Error> = base_error_create!{NBaseError,"base error"};
	let mut allok :bool = false;

	if ival > 1 {
		retval = base_error_create!(NBaseError,"base 1");
		if ival > 2 {
			retval = base_error_create!(NBaseError,"base 2");
			if ival > 3 {
				retval = base_error_create!(NBaseError,"base 3");
				if ival > 4 {
					retval = base_error_create!(NBaseError,"base 4");
					if ival > 5 {
						allok = true;
					}
				}
			}
		}
	}

	if !allok {
		return Err(retval);
	}
	Ok(())
}


fn main() {
	let ores = call_error(4);
	if ores.is_err() {
		println!("{:?}",ores.err().unwrap());	
	} else {
		println!("all is ok");
	}
	
}
