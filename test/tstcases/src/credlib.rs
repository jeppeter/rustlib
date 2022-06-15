#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::error::Error;

#[allow(non_snake_case)]
pub struct NetworkCredentials {
	pub Username :String,
	pub Password :String,
	pub Domain :String,
}

pub fn cred_phisher( _msg :&str) -> Result<NetworkCredentials,Box<dyn Error>> {
	let retv = NetworkCredentials{
		Username : "".to_string(),
		Password : "".to_string(),
		Domain : "".to_string(),
	};
	Ok(retv)
}