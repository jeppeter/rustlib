	fn call_2(c :u8) -> String {
			let s = format!("{}",c );
			//s.push_str(format!("{}",c ));
			return s;
		}
		pub fn call_3(c :u8) -> String {
			return call_2(c);
		}
