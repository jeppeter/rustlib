
use std::error::Error;
use std::net::Ipv4Addr;
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::*;

extargs_error_class!{NetLibError}

pub (crate) struct NetDevFd {
	fd :i32,
	ethname :String,
}

impl Drop for NetDevFd {
	fn drop(&mut self) {
		self.close();
	}
}

impl NetDevFd {
	fn close(&mut self) {
		if self.fd >= 0 {
			unsafe {
				libc::close(self.fd);
			}
			self.fd = -1;
		}
	}

	pub (crate) fn new(s :&str) -> Result<Self,Box<dyn Error>> {
		let mut retv :Self = Self {
			fd : -1,
			ethname :format!("{}",s),
		};
		unsafe {
			retv.fd = libc::socket(libc::AF_INET,libc::SOCK_DGRAM,0);
		}
		if retv.fd < 0 {
			let erri = get_errno!();
			extargs_new_error!{NetLibError,"can not socket error {}",erri}
		}
		Ok(retv)
	}


	pub (crate) fn get_netmask(&self) -> Result<String,Box<dyn Error>> {
		Ok("".to_string())
	}

	pub (crate) fn get_ipaddr(&self) -> Result<String,Box<dyn Error>> {
		Ok("".to_string())
	}

	pub (crate) fn get_default_gateway(&self) -> Result<String,Box<dyn Error>> {
		Ok("".to_string())
	}

	pub (crate) fn get_dns(&self) -> Result<Vec<String>,Box<dyn Error>> {
		let retv :Vec<String> = Vec::new();
		Ok(retv)
	}

}


pub fn format_sinaddr_in(ipaddr :&str,port :u32) -> Result<libc::sockaddr_in,Box<dyn Error>> {
	let mut retv :libc::sockaddr_in = unsafe {std::mem::zeroed()};
	let ipv4 :Ipv4Addr = ipaddr.parse()?;
	let octs :[u8; 4] = ipv4.octets();
	let mut cv :u32 = 0;
	let mut idx :usize=0;
	while idx < octs.len() {
		cv |= (octs[idx] as u32) << (8 * idx);
		idx += 1;
	}


	retv.sin_family = libc::AF_INET as u16;
	retv.sin_port = (port as u16).to_be();
	retv.sin_addr = libc::in_addr { s_addr: cv };
	return Ok(retv);
}

