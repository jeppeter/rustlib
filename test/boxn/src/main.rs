use std::error::Error;
use std::boxed::Box;
use regex::Regex;
use std::fmt;

macro_rules! error_class {
	($type:ident) => {
	#[derive(Debug,Clone)]
	pub struct $type {
		msg :String,		
	}

	#[allow(dead_code)]
	impl $type {
		fn create(c :&str) -> $type {
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

macro_rules! new_error {
	($type:ty,$($a:expr),*) => {
		{
		let mut c :String= format!("[{}:{}][{}]",file!(),line!(),stringify!($type));
		c.push_str(&(format!($($a),*)[..]));
		return Err(Box::new(<$type>::create(c.as_str())));
	  }
	};
}


#[derive(Clone)]
pub struct NameSpaceEx {

}

impl NameSpaceEx {
	pub fn new() -> NameSpaceEx {
		NameSpaceEx{}
	}
	pub fn get_bool(&self, _k :&str) -> bool {
		return false;
	}
	pub fn get_int(&self,_k :&str) -> i64 {
		return 0;
	}
	pub fn get_float(&self,_k :&str) -> f64 {
		return 0.0;
	}
	pub fn get_array(&self,_k :&str) -> Vec<String> {
		return Vec::new();
	}
}

pub trait ArgSet {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>>;
}


#[derive(Debug)]
struct Dimension {
	bb :f64,
	cc :f64,
}

error_class!{DimensionError}

impl ArgSet for Dimension {
	fn set_value(&mut self,k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		if k == "bb" {
			self.bb = ns.get_float(k);
		} else if k == "cc" {
			self.cc = ns.get_float(k);
		} else {
			new_error!{DimensionError,"{} not support", k}
		}
		Ok(())
	}

}



#[derive(Debug)]
struct PoinX {
	x :f64,
	y :f64,
	next :Option<Box<PoinX>>,
	bs : Dimension,
}

error_class!{PoinXError}

impl PoinX {
	fn new(x1:f64,y1:f64) -> PoinX {
		PoinX{x:x1,y:y1,next:None, bs :Dimension {
			bb :x1,
			cc :y1,
		}}		
	}
	fn add_next(&mut self,v :Option<Box<PoinX>>) -> &PoinX{
		self.next = v;
		self
	}
	fn get_x(&self) -> f64 {
		self.x
	}
	fn get_y(&self) -> f64 {
		self.y
	}
}



impl ArgSet for PoinX {
	fn set_value(&mut self, k :&str, ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
		if k == "x" {
			self.x = ns.get_float("x");
		} else if k == "y" {
			self.y = ns.get_float("y");
		} else if k.starts_with("bs.") {
			let nk = format!("{}",k);
			let re = Regex::new(r"^bs\.").unwrap();
			let kn = re.replace_all(&nk,"").to_string();
			println!("kn {}", kn);
			self.bs.set_value(&kn,ns.clone())?;
		} else {
			new_error!{PoinXError,"[{}] not support", k}
		}
		Ok(())
	}
}

fn main() {
    let mut xc :Box<PoinX> = Box::new(PoinX::new(1.1,1.1));
    let mut c :Box<PoinX> = Box::new(PoinX::new(2.2,2.2));
    let bc :Box<PoinX> = Box::new(PoinX::new(3.3,3.3));
    let ns :NameSpaceEx = NameSpaceEx::new();
    c.add_next(Some(bc));
    xc.add_next(Some(c));
    xc.bs.bb = 2.2;
    xc.bs.cc = 3.2;
    xc.set_value("bs.bb",ns.clone()).unwrap();
    xc.set_value("bs.cc",ns.clone()).unwrap();
    println!("xc {:?} x {} y {}", xc,xc.get_x(),xc.get_y());
}
