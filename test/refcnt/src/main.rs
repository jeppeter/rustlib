use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
	next :Option<Rc<RefCell<Node>>>,
	head :Option<Weak<RefCell<Node>>>,
	c :Option<Rc<CallName>>
}

impl  Drop for Node {
	fn drop(&mut self) {
		println!("drop {:?}", self);
	}
}

#[allow(dead_code)]
#[derive(Debug)]
struct CallName {
	name :String,
}

impl Drop for CallName {
	// add code here
	fn drop(&mut self) {
		println!("drop {:?}", self);
	}
}

fn main()  {
	let a = Rc::new(CallName{name:"cc".to_string()});
	let b = a.clone();
	let first = Rc::new(RefCell::new(Node {next : None,head : None, c:None}));
	let second = Rc::new(RefCell::new(Node {next :None,head : None, c:None}));
	let third = Rc::new(RefCell::new(Node {next :None,head :None, c:None}));
	first.borrow_mut().next = Some(second.clone());
	first.borrow_mut().c = Some(a.clone());
	second.borrow_mut().next = Some(third.clone());
	second.borrow_mut().c = Some(b.clone());
	third.borrow_mut().head = Some(Rc::downgrade(&first));
	third.borrow_mut().c = Some(b.clone());
	println!("a {:?} b {:?}",a,b);
	println!("a {:?} b {:?}",a,b);
}