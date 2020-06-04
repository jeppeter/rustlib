
mod modcall {
	pub fn call_1(s :&str) -> &str{
		println!("input {}", s);
		return s;
	}

	pub mod nest {
		fn call_2(c :u8) -> String {
			let s = format!("{}",c );
			//s.push_str(format!("{}",c ));
			return s;
		}
		pub fn call_3(c :u8) -> String {
			return call_2(c);
		}
	}
}

fn main() {
	let s = String::from("xxx");
	let cs :String;
    modcall::call_1(&s);
    cs = modcall::nest::call_3(20);
    println!("{}", cs);

}
