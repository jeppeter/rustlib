
use std::error::Error;
use std::net::Ipv4Addr;

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

