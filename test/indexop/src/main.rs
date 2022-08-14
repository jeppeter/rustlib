use std::ops::{Index,IndexMut};

struct Mem<T> {
	v : Vec<T>,
}

impl<T> Mem<T> {
	fn new() -> Self {
		Mem {
			v : Vec::new(),
		}
	}

	fn push(&mut self, c :T) {
		self.v.push(c);
	}
}

impl<T> Index<usize> for Mem<T> {
	type Output = T;
	fn index(&self, index :usize) -> &Self::Output {
		if index >= self.v.len() {
			panic!("{} >= {}", index, self.v.len());
		}
		return &self.v[index];
	}
}

impl<T> IndexMut<usize> for Mem<T> {
	fn index_mut(&mut self,index :usize) -> &mut T {
		if index >= self.v.len() {
			panic!("{} >= {}", index, self.v.len());
		}
		return &mut self.v[index];
	}
}

fn main() {
	let mut c :Mem<i32> = Mem::new();
	c.push(32);
	c.push(60);
	println!("c[1] = {}",c[1] );
	c[1] = 55;
	println!("c[1] = {}",c[1] );
	c[2] = 55;
}

