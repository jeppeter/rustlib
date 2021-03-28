use winapi::um::synchapi::{CreateEventW,OpenEventW,WaitForSingleObject,SetEvent,WaitForMultipleObjects};
use std::env::args;
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};
use winapi::shared::ntdef::{HANDLE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{EVENT_ALL_ACCESS};
use winapi::um::winbase::{INFINITE,WAIT_OBJECT_0};
use std::{ptr,fmt};
use std::error::{Error};
use winapi::shared::minwindef::{FALSE,DWORD,TRUE,BOOL};
use winapi::um::errhandlingapi::{GetLastError};
use std::ffi::{OsStr};
use std::os::windows::ffi::{OsStrExt};
use std::mem;
//use lpwstr::ToWide;


const NULL_HANDLE :HANDLE = 0x0 as HANDLE;

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

		if self.value != INVALID_HANDLE_VALUE && self.value != NULL_HANDLE {
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

fn open_event(evtname :&str) ->Result<TokenHandle,EventError> {
	let hdl :HANDLE;

	unsafe {
		//hdl = OpenEventW(SYNCHRONIZE, FALSE, to_wide(evtname).as_ptr());	
		hdl = OpenEventW(EVENT_ALL_ACCESS, FALSE, to_wide(evtname).as_ptr());	
		if hdl == INVALID_HANDLE_VALUE || hdl == NULL_HANDLE {
			return Err(EventError::new(&(format!("create event [{}] error",evtname)[..])));
		} 
		return Ok(TokenHandle::from_raw_handle(hdl));
	}
}


fn create_event(evtname :&str) -> Result<TokenHandle,EventError> {
	let hdl :HANDLE;

	unsafe {
		hdl = CreateEventW(ptr::null_mut(), TRUE, FALSE, to_wide(evtname).as_ptr());	
		//hdl = CreateEventW(ptr::null_mut(), FALSE, FALSE, ptr::null());	
		if hdl == INVALID_HANDLE_VALUE || hdl == NULL_HANDLE {
			return Err(EventError::new(&(format!("create event [{}] error",evtname)[..])));
		} 
		return Ok(TokenHandle::from_raw_handle(hdl));
	}	
}

fn wait_event(evt :&TokenHandle, _timeout :i32) -> bool {
	let dret :DWORD;
	let wtime :DWORD = _timeout as DWORD;
	unsafe {
		dret = WaitForSingleObject(evt.as_raw_handle(),wtime);
		println!("dret {:?}", dret);
		if dret == WAIT_OBJECT_0 {
			return true;
		}
	}
	return false;
}

fn set_event(evt :&TokenHandle) -> bool {
	let bret :BOOL;
	unsafe {
		bret = SetEvent(evt.as_raw_handle());
		if  bret == FALSE {
			println!("set event {:?} error {:?}", bret,GetLastError());	
		}		
	}
	return true;
}

fn wait_events(evt :&Vec<TokenHandle>,_timeout :DWORD) -> Result<usize,EventError> {
	let dret :DWORD;
	let cnt :DWORD = evt.len() as DWORD;
	let mut _hevt :Vec<HANDLE> = Vec::new();
	let reti :usize;

	for i in 0..evt.len() {
		_hevt.push(evt[i].as_raw_handle());
	}

	unsafe {
		let hdls :*const HANDLE = _hevt.as_slice().as_ptr() as *const HANDLE;
		//let hdls :*const HANDLE = 0 as *const HANDLE;
		dret = WaitForMultipleObjects(cnt,hdls,FALSE,_timeout);
		if dret >= WAIT_OBJECT_0 && dret < (WAIT_OBJECT_0 + _hevt.len() as DWORD) {
			reti = (dret - WAIT_OBJECT_0) as usize;
			return Ok(reti);
		}
		return Err(EventError::new(&(format!("can not wait error [{:?}] last error[{:?}]",
			dret,GetLastError())[..])));
	}

}


fn main() {
	let argv:Vec<String> = args().collect();
	let evt :TokenHandle;
	let reti :usize;
	let mut vevt :Vec<TokenHandle> = Vec::new();
	let _bret :bool;
	if argv.len() > 1 {
		if argv[1] == "wait" {
			evt = create_event(&(argv[2])).unwrap();
			println!("create [{}] succ [{:?}]", argv[2],evt);
			//set_event(&evt);
			_bret = wait_event(&evt,INFINITE as i32);
			println!("wait [{}] event",argv[2]);
		} else if argv[1] == "setevent" {
			evt = open_event(&(argv[2])).unwrap();
			println!("open [{}] succ [{:?}]", argv[2],evt);
			set_event(&evt);
			println!("set event [{}]",argv[2]);
		} else if argv[1] == "waits" {
			for i in 2..argv.len() {
				vevt.push(create_event(&(argv[i])).unwrap());
			}
			println!("will wait [{:?}]",vevt);
			reti = wait_events(&vevt,INFINITE).unwrap();
			println!("wait get [{:?}]",vevt[reti]);
		} else if argv[1] == "vec" {
			let mut nums :Vec<i32> =  Vec::new();
			let narray :&[i32];
			for i in 2..argv.len() {
				nums.push(argv[i].parse::<i32>().unwrap());
			}
			narray = nums.as_slice();

			println!("{:?} size [{}] ptr [{:?}]", narray,mem::size_of_val(narray),narray.as_ptr());
		} else {
			eprintln!("{} not support command",argv[1] );

		}
	}
}