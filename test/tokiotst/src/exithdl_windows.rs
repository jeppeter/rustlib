
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extlog::{format_str_log,debug_trace};
use extlog::loglib::{log_get_timestamp,log_output_function};

use winapi::shared::minwindef::{BOOL,TRUE,FALSE};
use winapi::um::wincon::{CTRL_C_EVENT,CTRL_BREAK_EVENT};
use winapi::um::consoleapi::SetConsoleCtrlHandler;
use winapi::um::errhandlingapi::GetLastError;


use std::error::Error;
use crate::exithdl_consts::*;

extargs_error_class!{ExitHandleError}


struct CtrlHandleEvent {
	sender : tokio::sync::mpsc::UnboundedSender<u32>,
	events : Vec<u32>,
}

//lazy_static !{
static  mut EXIT_EVENTFD :Option<CtrlHandleEvent> = None;
//}


macro_rules! get_errno {
       () => {{
               let mut retv :i32 ;
               unsafe {
                       retv = GetLastError() as i32;
               }
               if retv != 0 {
                       retv = -retv;
               } else {
                       retv = -1;
               }
               retv
       }};
}



#[allow(static_mut_refs)]
unsafe extern "system" fn ctrl_c_handler(ty: u32) -> BOOL {
	debug_trace!("ty 0x{:x}",ty);
	if EXIT_EVENTFD.is_some() {
		let c = EXIT_EVENTFD.as_mut().unwrap();
		for v in c.events.iter() {
			if ty == *v {
				let _ = c.sender.send(ty);
				debug_trace!("set 0x{:x} event", *v);
			}
		}
	}
	return TRUE;
}


fn _get_exit_fd(sigv :Vec<u32>,tx :tokio::sync::mpsc::UnboundedSender<u32>) -> Option<CtrlHandleEvent> {
		let mut retv :CtrlHandleEvent = CtrlHandleEvent {
			sender : tx,
			events : Vec::new(),
		};
		let bret :BOOL;

		for v in sigv {
			let cvv = _trans_exit_value(v);
			if cvv != SIG_VALERR {
				retv.events.push(cvv);
			}
		}

		unsafe {
			bret = SetConsoleCtrlHandler(Some(ctrl_c_handler),TRUE);
		}
		if bret == FALSE {
			return None;
		}


		Some(retv)
}

fn _trans_exit_value(sigv :u32) -> u32 {
	let mut retv : u32 = SIG_VALERR;

	if sigv == SIG_INT {
		retv = CTRL_C_EVENT ;
	} else if sigv == SIG_TERM {
		retv = CTRL_BREAK_EVENT;
	}

	return retv;
}


#[allow(static_mut_refs)]
pub fn init_exit_handle(sigv :Vec<u32>,tx :tokio::sync::mpsc::UnboundedSender<u32>) -> Result<(),Box<dyn Error>> {
	unsafe {
		if EXIT_EVENTFD.is_none() {
			EXIT_EVENTFD = _get_exit_fd(sigv,tx);	
		}
		
		if EXIT_EVENTFD.is_some() {
			return Ok(());
		}
	}
	extargs_new_error!{ExitHandleError,"not init EXIT_EVENTFD {}",get_errno!()}
}


#[allow(dead_code)]
pub fn fini_exit_handle() {
	return;
}