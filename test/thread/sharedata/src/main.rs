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

pub struct Sharedata {
	pub exit : u8,
	pub thr : Option<JoinHandle<()>>,
}

impl Sharedata {
	pub fn new() -> Sharedata {
		Sharedata{ exit : 0, thr: None}
	}
	pub fn try_join(&mut self) -> bool {
			if self.exit == 1 {
				match self.thr {
					Some(v2) => {
						self.thr = None;
						v2.join().unwrap();
						return true;
					},
					None => {}
				}
			}
			return false;
	}

	pub fn join(&mut self) -> bool {
		if self.exit == 1 {
			match self.thr {
				Some(v2) => {
					self.thr = None;
					v2.join().unwrap();
					return true;
				},
				None => {

				}
			}
		} else {
			match self.thr {
				Some(v2) => {
					self.thr = None;
					v2.join().unwrap();
					self.exit = 1;
					return true;
				},
				None => {}
			}
		}
		return false;
	}
}


fn main() {

	let  w :Arc<Mutex<Writers>> = Arc::new(Mutex::new(Writers::new()));
	//let mut thrvec :Vec<Arc<JoinHandle<_>>> =  Vec::new();
	let mut shdata :Vec<Arc<Mutex<Sharedata>>> = Vec::new();

	for i in 0..10 {
		let cw = w.clone();
		let sw = Arc::new(Mutex::new(Sharedata::new()));
		let sd = sw.clone();

		let thr = thread::spawn(move || {
			{
				let mut cb = cw.lock().unwrap();
				cb.push_names(String::from(format!("thread {}", i)), String::from(format!("publish {}",i)));
				println!("thread {} {}",i,cb.description());
			}
			thread::sleep(time::Duration::from_millis(1000));
			{
				let mut cb = sd.lock().unwrap();
				cb.exit = 1;
			}
		});
		{
			let mut cb = sw.lock().unwrap();
			cb.thr = Some(thr);
		}
		shdata.push(sw);
	}

	for i in 0..shdata.len() {
		let v = shdata[i].clone();
		{
			let mut cb = v.lock().unwrap();
			if cb.exit == 1 {
				println!("[{}] exited", i);
				cb.try_join();
			} else {
				println!("[{}] not exited", i);
			}
		}
		thread::sleep(time::Duration::from_millis(500));
	}

	for i in 0..shdata.len() {
		let v = shdata[i].clone();
		{
			let mut cb = v.lock().unwrap();
			cb.join();
		}
	}
	{
		let cb = w.lock().unwrap();
		println!("descrip {}",  cb.description());
	}

	return;
}
