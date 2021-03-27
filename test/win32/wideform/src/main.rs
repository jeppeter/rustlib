use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::env::args;

fn wchar_to_string(s :&[u16]) -> String {
	return OsString::from_wide(s).to_string_lossy().into()     
}

fn main() {
	let mut i :i32;
	let mut s :Vec<u16> = Vec::new();
	i= 0;
	for c in args() {
		if i > 0 {
			s.push(c.parse::<u16>().unwrap());
		}
		i += 1;
	}

	println!("{:?} => {}", s, wchar_to_string(s.as_slice()));
}
