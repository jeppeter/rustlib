
#[derive(Debug)]
struct PoinX {
	x :f64,
	y :f64
}

fn new_poinx(x1:f64,y1:f64) -> PoinX{
	PoinX{x:x1,y:y1}
}

fn main() {
    let xc :Box<PoinX> = Box::new(new_poinx(2.1,32.21));
    println!("xc {:?}", xc);
}
