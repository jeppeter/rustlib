use std::env;

fn uc_first(n :&str) -> String {
	let cv :Vec<char> = n.chars().collect();
	let mut cidx :i32 =0;
	let mut rets :String = "".to_string();
	let bv :Vec<char> = n.to_uppercase().chars().collect();
	for c in cv.iter() {
		if cidx == 0 {
			rets.push(bv[0]);
		} else {
			rets.push(*c);
		}
		cidx += 1;
	}
	return rets;
}

fn main() {
    let mut i :i32 = 0;
    for a in env::args() {
    	if i > 0 {
    		let cs = uc_first(&a);
    		println!("{} => {}",a, cs);
    	}
    	i += 1;
    }
}
