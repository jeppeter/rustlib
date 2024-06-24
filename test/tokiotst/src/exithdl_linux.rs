
use std::error::Error;
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use super::exithdl_consts::{SIG_TERM,SIG_INT};

use extlog::{debug_trace,format_str_log};
use extlog::loglib::{log_get_timestamp,log_output_function};
//use lazy_static::lazy_static;

extargs_error_class!{SigHdlError}

//static HANDLE_EXIT :AtomicU64 = AtomicU64::new(INVALID_EVENT_HANDLE);
//static ST_SIGINITED :AtomicU16 = AtomicU16::new(0);


//lazy_static !{
static mut EXIT_EVENTFD :Option<tokio::sync::mpsc::UnboundedSender<u32>> = None;
//}


unsafe fn rust_signal(_iv :libc::c_int) {
	if  EXIT_EVENTFD.is_some() {
		let mut s = EXIT_EVENTFD.as_mut().unwrap().clone();
		let _ = s.send(_iv as u32);
		debug_trace!("signaled {}",_iv);
	}
	return;
}

unsafe extern "system" fn notice_signal(iv : libc::c_int)  {
	rust_signal(iv);
	return;
}

fn get_notice_signal() -> libc::sighandler_t {
	notice_signal as *mut libc::c_void as libc::sighandler_t
}

fn _trans_exit_value(sigv :u32) -> libc::c_int {
	let mut retv :libc::c_int = -1;

	if sigv == SIG_TERM {
		retv = libc::SIGTERM;
	} else if sigv == SIG_INT {
		retv = libc::SIGINT;
	}

	return retv;
}


fn _get_exit_fd(sigs :Vec<u32>,tx :tokio::sync::mpsc::UnboundedSender<u32>) -> Option<EventFd> {
	if bres.is_err() {
		return None;
	}

	for v in sigs {
		let reti = _trans_exit_value(v);
		if reti >= 0 {
			let sigret :libc::sighandler_t;
			unsafe {
				sigret = libc::signal(reti,get_notice_signal());
			}
			if sigret == libc::SIG_ERR {
				return None;
			}
		}
	}

	Some(tx.clone())
}


pub fn init_exit_handle(sigs :Vec<u32>,tx :tokio::sync::mpsc::UnboundedSender<u32>) -> Result<(),Box<dyn Error>> {
	unsafe {
		EXIT_EVENTFD = _get_exit_fd(sigs,tx);
		if EXIT_EVENTFD.is_some() {
			return Ok(());
		}
	}
		extargs_new_error!{SigHdlError,"not init exit event"}
}

#[allow(dead_code)]
pub fn fini_exit_handle() {
	return;
}
