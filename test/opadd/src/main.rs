use std::ops;
use std::fmt;

struct Point{
	x :u32,
	y :u32,
}

impl fmt::Debug for Point {
	fn fmt(&self,f :&mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Point").field("x",&self.x).field("y",&self.y).finish()
	}
}


impl ops::Add for Point {
	type Output = Point;
	fn add(self, rhs :Point) -> Point {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl ops::Sub for Point {
	type Output = Point;
	fn sub(self, rsh :Point) -> Point {
		Point {
			x: self.x - rsh.x,
			y: self.y - rsh.y,
		}
	}
}

impl Copy for Point {}
impl Clone for Point {
	fn clone(&self) -> Point {
		*self
	}
}

fn main() {
	let xpoint :Point = Point{x:10,y:10};
	let ypoint :Point = Point{x:20,y:30};
	println!("{:?} + {:?} = {:?}",xpoint,  ypoint,xpoint + ypoint );
	println!("{:?} - {:?} = {:?}",ypoint, xpoint,ypoint - xpoint );

}
