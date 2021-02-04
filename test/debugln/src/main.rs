
macro_rules! debugln (
	() => {
		let __s = String::from(format!("[{}:{}]",file!(),line!()));
		println!("{}",__s);
	};
	($x : expr $(, $more:expr)*) => (
		let mut __s = String::from(format!("[{}:{}]",file!(),line!()));
		__s += &(format!($x,$($more),*)[..]);
		println!("{}",__s);
	);
);

fn main() {
	println!("[{}:{}]:hello world",file!(),line!());
	debugln!("new line");
	debugln!("xx [{}] [{}]", 3, 2);
}
