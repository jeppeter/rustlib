
macro_rules! debugln (
	() => {
		println!("[{}:{}]",file!(),line!());
	};
	($x : expr $(, $more:expr)*) => (
		print!("[{}:{}]",file!(),line!());
		println!($x,$($more) ,*);
	);
);

fn main() {
	println!("[{}:{}]:hello world",file!(),line!());
	debugln!("new line");
	debugln!("xx [{}] [{}]", 3, 2);
}
