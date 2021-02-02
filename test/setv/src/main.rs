fn modify(ref mut v :& mut Vec<i32>) {
	println!("{:p}", v);
	v.push(42);
	return;
}

fn main() {
    let mut v = vec![1,2,3];
    modify(&mut v);
    println!("{:?}", v);
    return;
}
