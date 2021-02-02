fn modify(v :& mut Vec<i32>) {
	v.push(42);
	return;
}

fn main() {
    let mut v = vec![1,2,3];
    modify(&mut v);
    println!("{:?}", v);
    return;
}
