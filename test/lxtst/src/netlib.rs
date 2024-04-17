
use std::error::Error;
use std::net::Ipv4Addr;
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::*;

extargs_error_class!{NetLibError}

struct NetDevFd {
	fd :i32,
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

	pub (crate) fn new() -> Result<Self,Box<dyn Error>> {
		let mut retv :Self = Self {
			fd : -1,
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

	pub (crate) fn get_fd(&self) -> i32 {
		return self.fd
	}
}

fn _get_netdev_netmask(fd :i32) -> Result<String,Box<dyn Error>> {
	let mut rets :String;
}


pub fn get_netdev_netmask(ethname :&str) -> Result<String,Box<dyn Error>> {

}

pub fn get_netdev_gateway(ethname :&str) -> Result<String,Box<dyn Error>> {

}

pub fn get_netdev_ipaddr(ethname :&str) -> Result<String,Box<dyn Error>> {

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

