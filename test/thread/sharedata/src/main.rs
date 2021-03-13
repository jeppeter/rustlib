use std::thread;
use std::sync::{Mutex,Arc};

#[derive(Clone,Debug)]
pub struct Writers {
	pub names :Vec<String>,
	pub publishes :Vec<String>,
}

impl Writers {
	pub fn new() -> Writers {
		Writers {
			names : Vec::new(),
			publishes: Vec::new(),
		}
	}

	pub fn push_names(&mut self,n :String,p :String) {
		self.names.push(n);
		self.publishes.push(p);
	}


	pub fn description(&self) -> String {
		let mut s :String;
		s = format!("name : publish [");

		for i in 0..self.names.len() {
			if i > 0 {
				s += &(format!(",")[..]);
			}
			s += &(format!("{}:{}",self.names[i],self.publishes[i])[..]);
		}

		s += &(format!("]")[..]);
		return s ;
	}
}


fn main() {

	let  w :Arc<Writers> = Arc::new(Writers::new());
	let cw = w.clone();
	let thr = thread::spawn(move || {
		{
			let 
		}
		thread::sleep(1);
		}
	})
}
