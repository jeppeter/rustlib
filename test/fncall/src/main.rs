
fn add(i :i32, j :i32) -> i32 {
	println!("add function {}+{}={}", i,j,i+j);
	return i+j;
}

fn add_2(i :i32, j :i32) -> i32 {
	println!("add2 function {}+{}+2={}",i,j,i+j+2 );
	return i+j+2;
}

fn main() {
	let fnptr : fn(i32,i32) -> i32 = add;
	let fn2ptr = add_2;
	fnptr(3,2);
	fn2ptr(3,2);
	(fnptr)(3,10);
	(fn2ptr)(20,10);
}
