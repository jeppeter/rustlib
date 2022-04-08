


#[allow(dead_code)]
#[derive(Debug)]
struct PoinX {
	x :f64,
	y :f64,
	next :Option<Box<PoinX>>
}

impl PoinX {
	fn new(x1:f64,y1:f64) -> PoinX {
		PoinX{x:x1,y:y1,next:None}		
	}
	fn add_next(&mut self,v :Option<Box<PoinX>>) -> &PoinX{
		self.next = v;
		self
	}
}

fn main() {
    let mut xc :Box<PoinX> = Box::new(PoinX::new(1.1,1.1));
    let mut c :Box<PoinX> = Box::new(PoinX::new(2.2,2.2));
    let bc :Box<PoinX> = Box::new(PoinX::new(3.3,3.3));
    c.add_next(Some(bc));
    xc.add_next(Some(c));
    println!("xc {:?}", xc);
}
