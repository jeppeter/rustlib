
use std::error::Error;
use std::net::Ipv4Addr;
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::*;
use super::loglib::*;
use super::fileop::*;
use regex::Regex;

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
			retv.fd = libc::socket(libc::AF_INET,libc::SOCK_STREAM,0);
		}
		if retv.fd < 0 {
			let erri = get_errno!();
			extargs_new_error!{NetLibError,"can not socket error {}",erri}
		}
		Ok(retv)
	}

	fn _ioctl_get(&self,code :libc::c_ulong,note :&str) -> Result<String,Box<dyn Error>> {
		if self.fd < 0 {
			extargs_new_error!{NetLibError,"not valid fd"}
		}
		let mut ifr :libc::ifreq = unsafe {std::mem::zeroed()};
		let mut maxsize :usize = libc::IFNAMSIZ - 1;
		if self.ethname.as_bytes().len() < maxsize {
			maxsize = self.ethname.as_bytes().len();
		}
		let mut idx :usize = 0;
		let cpv :&[u8] = self.ethname.as_bytes();
		while idx < maxsize {
			ifr.ifr_name[idx] = cpv[idx] as libc::c_char;
			idx += 1;
		}

		let  cp :*mut libc::sockaddr_in = unsafe {&mut ifr.ifr_ifru.ifru_addr as *mut libc::sockaddr as *mut libc::sockaddr_in};
		unsafe {(*cp).sin_family = libc::AF_INET as u16};

		let mut reti :libc::c_int;
		let __bbpptr = &ifr as *const libc::ifreq;
		debug_buffer_trace!(__bbpptr,std::mem::size_of::<libc::ifreq>(),"ifr IFNAMSIZ {}",libc::IFNAMSIZ);
		unsafe {
			let _ptr = &mut ifr as *mut libc::ifreq;
			reti = libc::ioctl(self.fd,code,_ptr);
		}

		if reti < 0 {
			reti = get_errno!();
			extargs_new_error!{NetLibError,"get {} error {}",note,reti}
		}


		let ptr :* const libc::sockaddr_in = unsafe{&ifr.ifr_ifru.ifru_addr as *const libc::sockaddr as *const libc::sockaddr_in};
		let sinaddr :libc::in_addr_t = unsafe {(*ptr).sin_addr.s_addr};
		let mut rets :String = "".to_string();

		let mut idx :usize;
		idx = 0;
		while idx < 4 {
			let cv :u8 = ((sinaddr >> (idx * 8)) & 0xff) as u8;
			if idx > 0 {
				rets.push_str(".");
			}
			rets.push_str(&format!("{}",cv));
			idx += 1;
		}
		Ok(rets)
	}


	pub (crate) fn get_netmask(&self) -> Result<String,Box<dyn Error>> {
		return self._ioctl_get(libc::SIOCGIFNETMASK,"netmask");
	}

	pub (crate) fn get_ipaddr(&self) -> Result<String,Box<dyn Error>> {
		return self._ioctl_get(libc::SIOCGIFADDR,"ipaddr");
	}

	pub (crate) fn get_default_gateway(&self) -> Result<String,Box<dyn Error>> {
		let s = read_file("/proc/net/route")?;
		let sarr :Vec<&str> = s.split("\n").collect();
		let matchs = format!("^{}\\s+([0-9a-fA-F]+)\\s+([0-9a-fA-F]+)\\s+.*",self.ethname);
		let ores = Regex::new(&matchs);
		let matchexpr :Regex;
		if ores.is_err() {
			extargs_new_error!{NetLibError,"compile [{}] error [{:?}]",matchs,ores.err().unwrap()}
		}
		matchexpr = ores.unwrap();

		for l in sarr {
			let ob = matchexpr.captures(l);
			if ob.is_some() {
				let ov = ob.unwrap();
				if ov.len() >= 3 {
					let destres = u32::from_str_radix(&ov[1],16);
					let gateres = u32::from_str_radix(&ov[2],16);
					if destres.is_ok() && gateres.is_ok() {
						let destaddr :u32 = destres.unwrap();
						let gateaddr :u32 = gateres.unwrap();
						if destaddr == 0 {
							let mut rets :String = "".to_string();
							let mut idx :usize = 0;
							while idx < 4 {
								let cv :u8 = ((gateaddr >> (idx * 8)) & 0xff) as u8;
								if idx > 0 {
									rets.push_str(".");
								}
								rets.push_str(&format!("{}",cv));
								idx += 1;
							}
							return Ok(rets);
						}
					}
				}
			}
		}
		extargs_new_error!{NetLibError,"can not get gateway"}
	}

	pub (crate) fn get_dns(&self) -> Result<Vec<String>,Box<dyn Error>> {
		let mut retv :Vec<String> = Vec::new();
		let s = read_file("/etc/resolv.conf")?;
		let sarr :Vec<&str> = s.split("\n").collect();
		let matchs = format!("^nameserver\\s+([0-9\\.]+)");
		let ores = Regex::new(&matchs);
		let matchexpr :Regex;
		if ores.is_err() {
			extargs_new_error!{NetLibError,"compile [{}] error [{:?}]",matchs,ores.err().unwrap()}
		}
		matchexpr = ores.unwrap();
		for l in sarr.iter() {
			let ob = matchexpr.captures(l);
			if ob.is_some() {
				let ov = ob.unwrap();
				if ov.len() >= 2 {
					retv.push(format!("{}",&ov[1]));
				}
			}
		}

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

