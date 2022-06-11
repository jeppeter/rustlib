
fn parse_log_var(s :&str) -> (String,u64) {
	let sarr :Vec<&str> = s.split(",").collect();
	let mut fname :String;
	let mut fsize :u64 = 0;
	if sarr.len() > 1 {
		fname = format!("{}",sarr[0]);
		let bss :String = format!("{}",sarr[1]);
		let bs2 = &bss;
		let bs = bs2.as_bytes();
		let mut number :String = "".to_string();
		let mut unit :String = "".to_string();
		let mut n :usize = bs.len();
		match bs2.find(|c :char| !c.is_digit(10)) {
			Some(vn) => {n = vn},
			None => {},
		}
		let mut idx :usize = 0 ;
		while idx < n {
			number.push(bs[idx] as char);
			idx += 1;
		}

		while idx < bs.len() {
			unit.push(bs[idx] as char);
			idx += 1;
		}


		match number.parse::<u64>() {
			Ok(n) => {fsize = n},
			Err(_e) => {},
		}
		if unit == "b" || unit == "B" {
			fsize = fsize;
		} else if unit == "k" || unit == "K" {
			fsize *= 1024;
		} else if unit == "m" || unit == "M" {
			fsize *= 1024 * 1024;
		} else if unit == "g" || unit == "G" {
			fsize *= 1024 * 1024 * 1024;
		} else if unit == "t" || unit == "T" {
			fsize *= 1024 * 1024 * 1024 * 1024;
		}
	} else {
		fname = format!("{}",s);
	}
	return (fname,fsize);
}

fn main() {
	let mut i :i32 = 0;
    for a in std::env::args() {
    	if i > 0 {
    		let (fname, fsize) = parse_log_var(&a);
    		println!("fname {} fsize {}", fname, fsize);
    	}
    	i += 1;
    }
}
