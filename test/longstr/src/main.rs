
use std::env;

fn longest<'a>( astr :&'a str ,  bstr :&'a str) -> &'a str {
	if astr.len() > bstr.len() {
		astr
	} else {
		bstr
	}
}


fn main() {
	let args :Vec<String> = env::args().collect();
	if args.len() > 2 {
		let a = format!("{}",args[1]);
		{
			let b = format!("{}",args[2]);
			let c = longest(&a,&b);
			println!("a [{}] b [{}] longest [{}]", a,b,c);			
		}
	} else {
		println!("no a b");
	}
	return;
}
