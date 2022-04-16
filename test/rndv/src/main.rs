use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
	for i in 1..10 {
		println!("[{}]rand[{}]",i,rng.gen_range(0..10000));
	}
}
