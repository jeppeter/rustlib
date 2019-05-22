fn push_string( s : &mut String) -> &String {
	s.push_str("world");
	return s;
}

fn main() {
    let mut s = String::from("hello ");
    push_string(&mut s);
    println!("{}", s);
    return;
}
