use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;
use std::error::Error;
use std::borrow::BorrowMut;

#[derive(Clone,Default)]
struct Duck {
	c : i32,
}

#[derive(Clone,Default)]
struct Pig {
	c : i32,
}

pub trait Fly   {
	fn fly(&self) -> bool;
	fn ccfly(&mut self) -> bool;
}

impl Fly for Duck {
	fn fly(&self) -> bool  {
		println!("Duck fly");
		return true;
	}

	fn ccfly(&mut self) -> bool {
		println!("Duck ccfly");
		return true;
	}
}

impl Fly for Pig {
	fn fly(&self) -> bool {
		println!("Pig not fly");
		return false;
	}
	fn ccfly(&mut self) -> bool {
		println!("Pig ccfly");
		return true;
	}
}

fn fly_static<T: Fly>(s :T) -> bool {
	s.fly()
}

fn fly_dyn(s :Box<dyn Fly>) -> bool {
	s.fly()
}

fn fly_static_ptr<T: Fly>(s :&T) -> bool {
	s.fly()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn call_Fly(args : Option<Arc<RefCell<dyn Fly>>>) -> Result<(),Box<dyn Error>> {
	if args.is_some() {
		println!("some _ctx");
		let ctx = args.as_ref().unwrap().clone();
		let mut cv :Box<dyn Any> = Box::new(args.as_ref().unwrap().clone());
		{
			print_type_of(&cv);
		}
		return Ok(());
	} else {
		println!("none of args");
	}
	return Ok(());
}


fn main() {
	let pig = Rc::new(Pig{c:20});
	let duck = Rc::new(Duck{c:23});
	let cduck = Arc::new(RefCell::new(Duck{c:99}));
	fly_static::<Pig>((*pig).clone());
	fly_static::<Duck>((*duck).clone());
	fly_static_ptr::<Pig>(&(*pig));
	fly_static_ptr::<Duck>(&(*duck));
	//fly_dyn(Box::new(pig));
	//fly_dyn(Box::new(duck));
	call_Fly(Some(cduck));
	return;

}