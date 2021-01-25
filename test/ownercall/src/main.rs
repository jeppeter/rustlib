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

#[allow(unused_variables)]
fn main() {
	let a = A{a:3,b:2};
	let b = A{a:a.a,b:a.b};
	let mut v = [1,2,3];
	println!("{:?} {:?}", a,b);
	println!("{:?} {:?}", a,b);
	foo(v);
	println!("{:?}", v);
	foo_2(&mut v);
	println!("{:?}", v);
}
