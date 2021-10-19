use std::thread;
use std::thread::JoinHandle;
use std::time;
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

	let  w :Arc<Mutex<Writers>> = Arc::new(Mutex::new(Writers::new()));
	let mut thrvec :Vec<Box<JoinHandle<_>>> =  Vec::new();

	for i in 1..10 {
		let cw = w.clone();
		let thr = thread::spawn(move || {
			{
				let mut cb = cw.lock().unwrap();
				cb.push_names(String::from(format!("thread {}", i)), String::from(format!("publish {}",i)));
				println!("thread {} {}",i,cb.description());
			}
			thread::sleep(time::Duration::from_millis(1000));
		});
		thrvec.push(Box::new(thr));
	}

	for v in thrvec {
		v.join().unwrap();
	}
	{
		let cb = w.lock().unwrap();
		println!("descrip {}",  cb.description());
	}
	return;
}
