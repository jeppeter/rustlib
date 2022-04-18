use rand::Rng;
use bytes::{BytesMut,BufMut};


fn get_bytes(num :u32, basevec :&[u8]) -> String {
	let mut retm = BytesMut::with_capacity(num as usize);
	let mut rng = rand::thread_rng();
	let mut curi :usize;

	for _i in 0..num {
		curi = rng.gen_range(0..basevec.len());
		retm.put_u8(basevec[curi]);
	}
	let a = retm.freeze();
	String::from_utf8_lossy(&a).to_string()
}

const RAND_NAME_STRING :[u8; 62]= *b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
	let mut rng = rand::thread_rng();
	for i in 1..10 {
		println!("[{}]rand[{}]",i,rng.gen_range(0..10000));
	}
	println!("ok [{}]",get_bytes(30,&RAND_NAME_STRING));
}
