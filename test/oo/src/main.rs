
struct Rectangle {
	width :u32,
	height :u32,
}


impl Rectangle {
	// add code here
	fn new(h :u32, w :u32) -> Rectangle {
		return Rectangle { width:w, height : h};
	}

	fn area(&self) -> u32 {
		return self.width * self.height;
	}
}

fn main() {
	let r :Rectangle;
	r = Rectangle::new(32,50);
	println!("area {}", r.area());
}
