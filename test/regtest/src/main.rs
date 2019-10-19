use regex::Regex;


fn main() {
	let re = Regex::new(r"(?x)
	(?P\d{4})
	(?P\d{2})

	(?P\d{2})
	").unwrap();
	let caps = re.captures("2010-03-14").unwrap();

	assert_eq!("2010", &caps["year"]);
	assert_eq!("03", &caps["month"]);
	assert_eq!("14", &caps["day"]);

}