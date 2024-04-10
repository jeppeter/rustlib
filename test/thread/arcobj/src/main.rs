use std::sync::{RwLock,Arc};
use std::time::Instant;
use std::backtrace::Backtrace;



struct CallInner {	
	name : String,
}

impl Drop for CallInner {
	fn drop(&mut self) {
		self.close();
	}
}


impl CallInner {

	pub fn new(n :&str) -> Arc<RwLock<Self>> {
		let retv :Self = Self {
			name : format!("{}",n),
		};
		retv.debug_self(file!(),line!());
		return Arc::new(RwLock::new(retv));
	}

	pub fn debug_self(&self,fname :&str,line :u32) {
		println!("[{}:{}]CallInner [{}]  [{:p}]",fname,line,self.name,self);
	}
	pub fn close(&mut self) {
		println!("{}", Backtrace::force_capture());
		self.debug_self(file!(),line!());
		println!("CallInner close {:p}", self);
	}

	pub fn get_name(&self) -> String {
		return format!("{}",self.name);
	}
}

#[derive(Clone)]
struct Call {
	inner :Arc<RwLock<CallInner>>,
}

impl Drop for Call {
	fn drop(&mut self) {
		self.close();
	}
}


impl Call {

	pub fn new(n :&str) -> Self {
		let retv :Self = Self {
			inner : CallInner::new(n),
		};
		retv.debug_self(file!(),line!());
		return retv;
	}

	pub fn debug_self(&self,fname :&str,line :u32) {
		let name :String;
		let cnt :usize;
		{
			let cv = self.inner.read().unwrap();
			name = cv.get_name();
			cnt = Arc::strong_count(&self.inner);
		}
		println!("[{}:{}]Call [{}] cnt [{}] [{:p}]",fname,line,name,cnt,self);
	}
	pub fn close(&mut self) {
		self.debug_self(file!(),line!());
		println!("Call close {:p}", self);
	}

	pub fn get_name(&self) -> String {
		let cv = self.inner.read().unwrap();
		return cv.get_name();
	}
}



fn thread_call(b :Call , c :Call,mills :i32) {
	let now = Instant::now();
	let wmills :u128 = mills as u128;
	let mut cnt :u32 = 0;
	loop {
		let curmills = now.elapsed().as_millis();
		if curmills > wmills {
			break;
		}

		std::thread::sleep(std::time::Duration::from_millis(10));
		cnt += 1;

		if (cnt % 100) == 0 {
			println!("[{}]child b [{}] c [{}]",cnt,b.get_name(),c.get_name());
			b.debug_self(file!(),line!());
			c.debug_self(file!(),line!());
		}
	}

	println!("child last b [{}] c [{}]",b.get_name(),c.get_name());
	b.debug_self(file!(),line!());
	c.debug_self(file!(),line!());
	return ;
}

fn main() {
	let bc :Call = Call::new("b");
	let cc :Call = Call::new("c");
	let b = bc.clone();
	let c = cc.clone();
	let chldmills :i32 = 2000;
	let parentmills :i32 = 3000;

	let o = std::thread::spawn(move || {
		thread_call(b,c,chldmills);
	});
	let now = Instant::now();
	let wmills :u128 = parentmills as u128;
	let cwmills :u128 = chldmills as u128;
	let mut cnt :u32 = 0;

	loop {
		let curmills = now.elapsed().as_millis();
		if curmills > wmills {
			break;
		}

		std::thread::sleep(std::time::Duration::from_millis(10));
		cnt += 1;

		if (cnt % 100) == 0 {
			println!("[{}]parent b [{}] c [{}]",cnt,bc.get_name(),cc.get_name());
			bc.debug_self(file!(),line!());
			cc.debug_self(file!(),line!());
		}
	}

	println!("[{}]parent after b [{}] c [{}]",cnt,bc.get_name(),cc.get_name());
	bc.debug_self(file!(),line!());
	cc.debug_self(file!(),line!());

	loop {
		let curmills = now.elapsed().as_millis();
		if curmills > cwmills {
			break;
		}
		std::thread::sleep(std::time::Duration::from_millis(10));
		cnt += 1;

		if (cnt % 100) == 0 {
			println!("[{}]parent wait b [{}] c [{}]",cnt,bc.get_name(),cc.get_name());
			bc.debug_self(file!(),line!());
			cc.debug_self(file!(),line!());
		}
		
	}
	o.join().unwrap();
	return;
}
