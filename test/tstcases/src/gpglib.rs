

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use lazy_static::lazy_static;
use std::error::Error;

const GPG_CRC24_INIT :u32 = 0xB704CE;
const GPG_CRC24_POLY :u32 = 0x864CFB;


extargs_error_class!{GpgLibError}

lazy_static !{
	static ref GPG_CRC24_TABLE :Vec<u32> = {
		let mut retv :Vec<u32> = Vec::new();
		let mut i :usize;
		for _ in 0..256 {
			retv.push(0);
		}
		i = 0;
		for j in 0..128 {
			let mut t :u32 ;
			t = retv[j];
			if (t & 0x00800000) != 0 {
				t <<= 1;
				retv[i] = t ^ GPG_CRC24_POLY;
				i += 1;
				retv[i] = t;
				i += 1;
			} else {
				t <<= 1;
				retv[i] = t;
				i += 1;
				retv[i] = t ^ GPG_CRC24_POLY;
				i += 1;
			}
		}

		retv
	};
}

pub struct GpgCrc24 {
	crc : u32,	
}


impl GpgCrc24 {
	pub fn new() -> GpgCrc24 {
		GpgCrc24 {
			crc : GPG_CRC24_INIT,
		}
	}

	pub fn get(&self) -> u32 {
		return self.crc;
	}

	pub fn update(&mut self, buf :&[u8]) {
		let mut crc :u32 = self.crc;
		for i in 0..buf.len() {
			let cb :u8 = ((crc >> 16) & 0xff) as u8;
			let cidx : usize = (cb ^ (buf[i])) as usize;
			crc = (crc << 8) ^ GPG_CRC24_TABLE[cidx];
		}
		self.crc = crc & 0x00ffffff;
	}
}

pub trait GpgPackage {
	fn init_gpg() -> Self;
	fn decode_gpg(&mut self, code :&[u8]) -> Result<usize,Box<dyn Error>>;
	fn encode_gpg(&self) -> Result<usize,Box<dyn Error>>;
}

#[derive(Debug)]
pub struct GpgPkg {
	pub code :u8,
	pub hdrlen :usize,
	pub data :Vec<u8>,
}


#[allow(dead_code)]
impl GpgPkg {
	pub fn new() -> GpgPkg {
		GpgPkg {
			code : 0,
			hdrlen : 0,
			data : Vec::new(),
		}
	}

	pub fn pack(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut retv :Vec<u8> = Vec::new();
		if self.hdrlen > 0 && self.data.len() > 0 {
			let mut datalen : usize = self.data.len();
			retv.push(self.code);
			if (self.code & 0x40) != 0 {
				/*new code header*/
				if datalen < 192 {
					retv.push(( datalen & 0xff )as u8);
				} else if datalen < 8384 {
					datalen -= 192;
					retv.push((datalen / 256 + 192) as u8);
					retv.push((datalen % 256) as u8);
				} else {
					retv.push(0xff);
					retv.push((datalen >> 24) as u8);
					retv.push((datalen >> 16) as u8);
					retv.push((datalen >> 8) as u8);
					retv.push((datalen & 0xff) as u8);
				}
			} else {
				if datalen < 256 {
					if (self.code & 0x3 ) != 0 {
						extargs_new_error!{GpgLibError,"code [0x{:02x}] not valid for datalen [0x{:x}]", self.code,datalen}
					}
					retv.push((datalen) as u8);
				} else if datalen < (1 << 16) {
					if (self.code & 0x3) != 2 {
						extargs_new_error!{GpgLibError,"code [0x{:02x}] not valid for datalen [0x{:x}]", self.code,datalen}	
					}
					retv.push(((datalen >> 8) & 0xff) as u8);
					retv.push((datalen & 0xff) as u8);
				} else {
					if (self.code & 0x3) != 3 {
						extargs_new_error!{GpgLibError,"code [0x{:02x}] not valid for datalen [0x{:x}]", self.code,datalen}	
					}
					retv.push(((datalen >> 24) & 0xff) as u8);
					retv.push(((datalen >> 16) & 0xff) as u8);
					retv.push(((datalen >> 8) & 0xff) as u8);
					retv.push(((datalen >> 0) & 0xff) as u8);
				}
			}

			for i in 0..self.data.len(){ 
				retv.push(self.data[i]);
			}
		}

		Ok(retv)
	}

	pub fn unpack(&mut self, code :&[u8]) -> Result<usize,Box<dyn Error>> {
		let cod :u8 ;
		let hdrlen :usize;
		let mut datalen :usize = 0;

		if code.len() < 2 {
			extargs_new_error!{GpgLibError,"code length [{}] < 2" ,code.len()}
		}

		if (code[0] & 0x40) != 0 {
			cod = code[0];
			/*new format*/
			if code[1] == 0xff {
				/*that is 4 bytes code*/
				if code.len() < 6 {
					extargs_new_error!{GpgLibError,"code length [{}] < 6" ,code.len()}
				}
				datalen |= (code[2] as usize) << 24;
				datalen |= (code[3] as usize) << 16;
				datalen |= (code[4] as usize) << 8;
				datalen |= (code[5] as usize) << 0;
				hdrlen = 6;
			} else if code[1] >= 192 {
				if code.len() < 3 {
					extargs_new_error!{GpgLibError,"code length [{}] < 3" ,code.len()}	
				}
				datalen += 192;
				datalen += ((code[1] - 192) as usize) * 256;
				datalen += code[2] as usize;
				hdrlen = 3;
			} else {
				datalen += code[2] as usize;
				hdrlen = 2;
			}
		} else {
			cod = code[0];
			if (cod & 0x3) == 3 {
				/**/
				if code.len() < 5 {
					extargs_new_error!{GpgLibError,"code length [{}] < 5" ,code.len()}	
				}
				hdrlen = 5;
				datalen |= (code[1] as usize) << 24;
				datalen |= (code[2] as usize) << 16;
				datalen |= (code[3] as usize) << 8;
				datalen |= code[4] as usize ;
			} else if (cod & 0x1) == 1 {
				if code.len() < 3 {
					extargs_new_error!{GpgLibError,"code length [{}] < 3" ,code.len()}	
				}
				hdrlen = 3;
				datalen |= (code[1] as usize) << 8;
				datalen |= code[1] as usize;
			} else {
				hdrlen = 2;
				datalen += code[1] as usize;
			}
		}

		if (hdrlen + datalen ) > code.len() {
			extargs_new_error!{GpgLibError,"hdrlen [{}] + datalen [{}] > code [{}]", hdrlen,datalen,code.len()}
		}

		self.code = cod;
		self.hdrlen = hdrlen;
		self.data = Vec::new();
		for i in 0..datalen {
			self.data.push(code[(hdrlen+i)]);
		}
		let retv :usize = hdrlen + datalen;
		Ok(retv)
	}
}

pub struct GpgSymKeyEnc {

}
