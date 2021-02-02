use std::env;

fn modify(v :& mut Vec<i32>) {
	println!("{:p}", v);
	v.push(42);
	return;
}

fn main() {
    let mut v = vec![1,2,3];
    let args :Vec<String> = env::args().collect();
    let mut bs :&[u8];
    //let mut i :i32;
    modify(&mut v);
    println!("{:?}", v);

    if args.len() > 1 {
    	for j in 1..args.len() {
	    	bs = args[j].as_bytes();
	    	for i in 0..bs.len() {
	    		println!("[{}][{}]=[{:x}]",j,i,bs[i]);
	    	}    		
    	}
    }
    return;
}
