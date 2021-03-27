use winapi::um::synchapi::{CreateEventW};
use std::env::args;
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};
use winapi::shared::ntdef::{HANDLE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use std::{ptr,fmt};
use std::error::{Error};
use winapi::shared::minwindef::{FALSE};
use std::ffi::{OsStr};
use std::os::windows::ffi::{OsStrExt};
//use lpwstr::ToWide;


#[derive(Debug)]
struct EventError {
    details: String
}

impl EventError {
    fn new(msg: &str) -> EventError {
        EventError{details: msg.to_string()}
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for EventError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct TokenHandle {
	value: HANDLE,
}

impl AsRawHandle for TokenHandle {
	fn as_raw_handle(&self) -> HANDLE {
		self.value
	}
}

impl FromRawHandle for TokenHandle {
	unsafe fn from_raw_handle(handle: HANDLE) -> TokenHandle {
		TokenHandle { value: handle }
	}
}

impl IntoRawHandle for TokenHandle {
	fn into_raw_handle(mut self) -> HANDLE {
		let value = self.value;
		self.value = INVALID_HANDLE_VALUE;
		value
	}
}

impl Drop for TokenHandle {
	fn drop(&mut self) {
		let nullhdl :HANDLE = 0x0 as HANDLE;

		if self.value != INVALID_HANDLE_VALUE && self.value != nullhdl {
			unsafe { CloseHandle(self.value); }
		}
		self.value = INVALID_HANDLE_VALUE;
	}
}

//fn to_u16(s :&str) -> Vec<u16> {
//	return OsStr::new(&s).encode_wide().collect();
//}

fn to_wide(s :&str) -> Vec<u16>  {
	return OsStr::new(s).encode_wide().chain(Some(0)).collect();
}


fn create_event(evtname :&str) -> Result<TokenHandle,EventError> {
	let hdl :HANDLE;
	let nullhdl :HANDLE = (0x0 as HANDLE);

	unsafe {
		hdl = CreateEventW(ptr::null_mut(), FALSE, FALSE, to_wide(evtname).as_ptr());	
		//hdl = CreateEventW(ptr::null_mut(), FALSE, FALSE, ptr::null());	
		if hdl == INVALID_HANDLE_VALUE || hdl == nullhdl {
			return Err(EventError::new(&(format!("create event [{}] error",evtname)[..])));
		} 
		println!("create {:?}", hdl);
		return Ok(TokenHandle::from_raw_handle(hdl));
	}	
}

fn main() {
	let argv:Vec<String> = args().collect();
	let evt :TokenHandle;
	if argv.len() > 1 {
		if argv[1] == "create" {
			evt = create_event(&(argv[2])).unwrap();
			println!("create [{}] succ [{:?}]", argv[2],evt);
		} else {
			eprintln!("{} not support command",argv[1] );

		}
	}
}