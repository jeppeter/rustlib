use regex::Regex;

fn capture_regex(restr :&str, instr :&str) {
	let re = Regex::new(restr).unwrap();
	let caps = re.captures(instr);
	match caps {
		Some(v) => {println!("capture {:?}", v);}
		None => {println!("error {:?} {:?}", restr,instr);}
	}

}

fn main() {
	capture_regex(r"(\d+)-(\d+)-(\d+)","dds2")
}