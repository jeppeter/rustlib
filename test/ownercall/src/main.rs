#[derive(Debug)]
struct A {
	a :i32,
	b :u32,
}

fn foo(mut v :[i32 ;3]) -> [i32 ;3] {
	v[0] = 3;
	return v;
}

fn foo_2 ( v:& mut[i32 ;3]) {
	v[0] = 3;
}

#[derive(Debug)]
struct User {
	name :String,
	url :String,
}

fn create_user(n :&str,c :&str) -> User {
	return User{ name : String::from(n), url : String::from(c)};
}

impl User {
	fn show(&self) {
		println!("name={};url={}",self.name,self.url );
	}
}
impl Drop for User {
	fn drop(&mut self) {
		println!("drop {};{}", self.name,self.url);
		self.name = String::from("");
		self.url = String::from("");
	}	
}

#[allow(unused_variables)]
fn main() {
	let a = A{a:3,b:2};
	let b = A{a:a.a,b:a.b};
	let mut v = [1,2,3];
	let mut c = create_user("xx","rrw");
	println!("{:?} {:?}", a,b);
	println!("{:?} {:?}", a,b);
	foo(v);
	println!("{:?}", v);
	foo_2(&mut v);
	println!("{:?}", v);
	c.show();
}
