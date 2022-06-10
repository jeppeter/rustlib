use chrono::{Local,Datelike,Timelike};

fn main() {
	let now = Local::now();
	println!("now {}/{}/{} {}:{}:{}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second());
}
