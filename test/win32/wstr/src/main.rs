use std::env::args;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;


fn to_u16(s :&str) -> Vec<u16> {
	return OsStr::new(&s).encode_wide().collect();
}

fn to_u8(s :&str) -> Vec<u8> {
	return s.as_bytes().to_vec();
}

fn main() {
	let mut i :i32;
	let mut s :String;
	let mut icnt :i32;
	i = 0;
	for c in args() {
		s = "".to_string();
		s.push_str(&(format!("[{}]=[{}][",i,c)[..]));
		icnt = 0;

		for ic in to_u8(&c) {
			if icnt > 0 {
				s += &(format!(",")[..]);
			}
			s += &(format!("0x{:x}",ic )[..]);
			icnt += 1;
		}
		s += &(format!("]wide16[")[..]);
		icnt = 0;
		for ic in to_u16(&c) {
			if icnt > 0 {
				s += &(format!(",")[..]);
			}
			s += &(format!("0x{:x}",ic)[..]);
			icnt += 1;
		}
		s += &(format!("]")[..]);


		println!("{}", s);
		i += 1;
	}
}
