use std::rc::Rc;

#[derive(Clone)]
struct Duck;
#[derive(Clone)]
struct Pig;

trait Fly {
	fn fly(&self) -> bool;
}

impl Fly for Duck {
	fn fly(&self) -> bool  {
		println!("Duck fly");
		return true;
	}
}

impl Fly for Pig {
	fn fly(&self) -> bool {
		println!("Pig not fly");
		return false;
	}
}

fn fly_static<T: Fly>(s :T) -> bool {
	s.fly()
}

fn fly_dyn(s :&dyn Fly) -> bool {
	s.fly()
}

fn main() {
	let pig = Rc::new(Pig);
	let duck = Rc::new(Duck);
	fly_static::<Pig>(*pig);
	fly_static::<Duck>(*duck);
	fly_dyn(&(*pig));
	fly_dyn(&(*duck));
	return;

}