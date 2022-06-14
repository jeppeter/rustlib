
use std::slice;
use std::ffi::OsStr;
use winreg::enums::*;
use winreg::{RegValue,RegKey};
use winapi::um::winreg as winapi_reg;
use std::os::windows::ffi::OsStrExt;
use std::error::Error;
use std::collections::HashMap;

pub const REG_HKLM :&str = "HKLM";
pub const REG_HCU :&str = "HCU";
pub const REG_HKCR :&str = "HKCR";
pub const REG_HKU :&str = "HKU";
pub const REG_HKCC :&str = "HKCC";

pub const TYPE_REG_SZ :&str = "REG_SZ";
pub const TYPE_REG_EXPAND_SZ :&str = "REG_EXPAND_SZ";
pub const TYPE_REG_BINARY :&str = "REG_BINARY";
pub const TYPE_REG_DWORD :&str = "REG_DWORD";
pub const TYPE_REG_QWORD :&str = "REG_QWORD";
pub const TYPE_REG_DWORD_BIG_ENDIAN :&str = "REG_DWORD_BIG_ENDIAN";
pub const TYPE_REG_MULTI_SZ :&str = "REG_MULTI_SZ";

fn get_regk(ktype :&str) -> (RegKey,usize) {
	let mut step :usize = 0;
	let retk :RegKey;
	if ktype == REG_HKLM {
		retk = RegKey::predef(HKEY_LOCAL_MACHINE);
		step += 1;
	} else if ktype == REG_HCU {
		retk = RegKey::predef(HKEY_CURRENT_USER);
		step += 1;
	} else if ktype == REG_HKCR {
		retk = RegKey::predef(HKEY_CLASSES_ROOT);
		step += 1;
	} else if ktype == REG_HKCC {
		retk = RegKey::predef(HKEY_CURRENT_CONFIG);
		step += 1;
	} else if ktype == REG_HKU {
		retk = RegKey::predef(HKEY_USERS);
		step += 1;
	} else {
		retk = RegKey::predef(HKEY_LOCAL_MACHINE);
	}
	return (retk,step);
}

fn main_to_utf16<P: AsRef<OsStr>>(s: P) -> Vec<u16> {
	s.as_ref()
	.encode_wide()
	.chain(Some(0).into_iter())
	.collect()
}

fn main_v16_to_v8(v: &[u16]) -> Vec<u8> {
	unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 2).to_vec() }
}

pub fn open_reg_key(ktype :&str,kpath :&str, perms :winapi_reg::REGSAM) -> Result<RegKey,Box<dyn Error>> {
	let (regk,_) =  get_regk(ktype);
	let ckey :RegKey = regk.open_subkey_with_flags(kpath,perms)?;
	return Ok(ckey);
}

pub fn get_reg_keys(k :&RegKey) -> Vec<String> {
	let mut retv :Vec<String> = Vec::new();
	for kv in k.enum_keys() {
		retv.push(format!("{}",kv.unwrap()));
	}
	retv
}

pub fn get_reg_values(k :&RegKey) -> HashMap<String,RegValue> {
	let mut retv :HashMap<String,RegValue> = HashMap::new();
	for kp in k.enum_values() {
		let (kn,kv) = kp.unwrap();
		let nk :String = format!("{}",kn);
		let nv :RegValue = RegValue {
			bytes : kv.bytes.clone(),
			vtype : kv.vtype,
		};
		retv.insert(nk,nv);
	}	
	retv
}

pub fn get_reg_val(v :Vec<String>) -> RegValue {
	let mut idx :usize;

	if v.len() > 0 {
		if v[0] == TYPE_REG_SZ {
			if v.len() == 1 {
				return RegValue { 
					bytes : main_v16_to_v8(&main_to_utf16("\0")),
					vtype : REG_SZ,
				};
			}
			let cstr = format!("{}",v[1]);

			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(cstr)),
				vtype : REG_SZ,
			};
		} else if v[0] == TYPE_REG_EXPAND_SZ {
			if v.len() == 1 {
				return RegValue { 
					bytes : main_v16_to_v8(&main_to_utf16("\0")),
					vtype : REG_EXPAND_SZ,
				};
			}
			let cstr = format!("{}",v[1]);

			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(cstr)),
				vtype : REG_EXPAND_SZ,
			};
		} else if v[0] == TYPE_REG_BINARY {
			let mut retb :Vec<u8> = Vec::new();
			if v.len() == 1 {
				retb.push(0 as u8);
				return RegValue {
					bytes : retb,
					vtype : REG_BINARY,
				};
			}

			idx = 1;
			while idx < v.len() {
				retb.push(v[idx].parse::<u8>().unwrap());
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_BINARY,
			};
		} else if v[0] == TYPE_REG_DWORD {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u32 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u32>().unwrap();
			}
			idx = 0;
			while idx < 4 {
				let curb :u8 = ((dval >> (idx * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_DWORD,
			};
		} else if v[0] == TYPE_REG_QWORD {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u64 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u64>().unwrap();
			}
			idx = 0;
			while idx < 8 {
				let curb :u8 = ((dval >> (idx * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_QWORD,
			};
		} else if v[0] == TYPE_REG_DWORD_BIG_ENDIAN {
			let mut retb :Vec<u8>=Vec::new();
			let mut dval :u32 = 0;
			if v.len() > 1 {
				dval = v[1].parse::<u32>().unwrap();
			}
			idx = 0;
			while idx < 4 {
				let curb :u8 = ((dval >> ((3-idx) * 8))  & 0xff) as u8;
				retb.push(curb);
				idx += 1;
			}
			return RegValue {
				bytes : retb,
				vtype : REG_DWORD_BIG_ENDIAN,
			};			
		} else if v[0] == TYPE_REG_MULTI_SZ {
			let mut s :String = "".to_string();

			if v.len() > 1 {
				idx = 1;
				while idx < v.len() {
					s.push_str(&v[idx]);
					idx += 1;
				}
			}
			return RegValue {
				bytes : main_v16_to_v8(&main_to_utf16(&s)),
				vtype : REG_MULTI_SZ,
			};
		}

	}
	return RegValue {
		bytes : main_v16_to_v8(&main_to_utf16("\0")),
		vtype : REG_SZ,
	};
}