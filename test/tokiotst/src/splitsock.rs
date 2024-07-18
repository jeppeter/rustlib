#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::options::{ExtArgsOptions};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};

#[allow(unused_imports)]
use std::cell::{RefCell,UnsafeCell};
#[allow(unused_imports)]
use std::sync::{Arc,Mutex};
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;

#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use extlog::loglib::{log_get_timestamp,log_output_function};
use extlog::{debug_info,debug_trace,debug_error,debug_buffer_trace,format_buffer_log,format_str_log};

use crate::exithdl_consts::{SIG_TERM,SIG_INT};
use crate::exithdl::{init_exit_handle};
use crate::logtrans::{init_log};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


extargs_error_class!{SplitSockError}

struct SockHandleInner {
	rcv :tokio::sync::mpsc::UnboundedReceiver<(Arc<Mutex<Vec<u8>>>,u64)>,
	snds :Vec<tokio::sync::mpsc::UnboundedSender<Arc<Mutex<Vec<u8>>>>>,
	sndidx :Vec<u64>,
}

impl SockHandleInner {
	fn new(rcv :tokio::sync::mpsc::UnboundedReceiver<(Arc<Mutex<Vec<u8>>>,u64)>) -> Result<Self,Box<dyn Error>> {
		Ok(Self {
			rcv :rcv,
			snds :vec![],
			sndidx :vec![],
		})
	}

	fn add_snd(&mut self,snd :tokio::sync::mpsc::UnboundedSender<Arc<Mutex<Vec<u8>>>>,idx :u64) -> Result<(),Box<dyn Error>> {
		debug_assert!(self.snds.len() == self.sndidx.len(),"snds.len {} != sndidx.len {}",self.snds.len(),self.sndidx.len());
		self.snds.push(snd);
		self.sndidx.push(idx);
		Ok(())
	}

	fn remove_snd(&mut self,idx :u64) -> Result<i32,Box<dyn Error>> {
		let mut uidx :usize = 0;
		debug_assert!(self.snds.len() == self.sndidx.len(),"snds.len {} != sndidx.len {}",self.snds.len(),self.sndidx.len());
		while uidx < self.sndidx.len() {
			if self.sndidx[uidx] == idx {
				self.snds.remove(uidx);
				self.sndidx.remove(uidx);
				return Ok(1);
			}
			uidx += 1;
		}
		return Ok(0);
	}

	fn find_snd(&self,idx :u64) -> Option<tokio::sync::mpsc::UnboundedSender<Arc<Mutex<Vec<u8>>>>> {
		let mut uidx :usize = 0;
		debug_assert!(self.snds.len() == self.sndidx.len(),"snds.len {} != sndidx.len {}",self.snds.len(),self.sndidx.len());
		while uidx < self.sndidx.len() {
			if self.sndidx[uidx] == idx {
				return Some(self.snds[uidx].clone());
			}
			uidx += 1;
		}
		return None;
	}
	async fn receive_fn(&mut self) -> Result<(),Box<dyn Error>> {
		loop {
			let ores = self.rcv.recv().await;
			if ores.is_none() {
				continue;
			}
			let (data,idx) = ores.unwrap();
			let osnd = self.find_snd(idx);
			if osnd.is_none() {
				debug_trace!("can not get idx {}",idx);
				continue;
			}
			let snd = osnd.unwrap();
			snd.send(data);
		}
	}
}

struct SockHandle {
	inner :Arc<UnsafeCell<SockHandleInner>>,
}

impl SockHandle {
	fn new(rcv :tokio::sync::mpsc::UnboundedReceiver<(Arc<Mutex<Vec<u8>>>,u64)>) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner :Arc::new(UnsafeCell::new(SockHandleInner::new(rcv)?)),
		};
		Ok(retv)
	}

	fn add_snd(&mut self,snd :tokio::sync::mpsc::UnboundedSender<Arc<Mutex<Vec<u8>>>>,idx :u64) -> Result<(),Box<dyn Error>> {
		let s1 = unsafe {&mut *self.inner.get()};
		return s1.add_snd(snd,idx);
	}

	fn remove_snd(&mut self,idx :u64) -> Result<i32,Box<dyn Error>> {
		let s1 = unsafe {&mut *self.inner.get()};
		return s1.remove_snd(idx);
	}

	async fn receive_fn(&mut self) -> Result<(),Box<dyn Error>> {
		let s1 = unsafe {&mut *self.inner.get()};
		return s1.receive_fn().await;
	}

}

async fn ctrl_recv(exitchl :&mut tokio::sync::mpsc::UnboundedReceiver<u32>) -> u32 {
	return exitchl.recv().await.unwrap();
}

#[allow(unreachable_code)]
async fn wsock_handle(mut wsock :tokio::net::tcp::OwnedWriteHalf,mut rx :tokio::sync::mpsc::UnboundedReceiver<Arc<Mutex<Vec<u8>>>>) -> Result<(),Box<dyn Error>> {
	loop {
		let ores = rx.recv().await;
		if ores.is_none() {
			continue;
		}
		let wbuf = ores.unwrap();
		{
			let cbuf = wbuf.lock().unwrap();
			debug_buffer_trace!(cbuf.as_ptr(),cbuf.len(),"will send buffer");
			let ores = wsock.write_all(&cbuf[0..cbuf.len()]).await;
			if ores.is_err() {
				debug_error!("write error {:?}",ores.err().unwrap());
				continue;
			}
		}
	}
	Ok(())
}

#[allow(unreachable_code)]
async fn rsock_handle(mut rsock :tokio::net::tcp::OwnedReadHalf,tx :tokio::sync::mpsc::UnboundedSender<(Arc<Mutex<Vec<u8>>>,u64)>,uidx :u64) -> Result<(),Box<dyn Error>> {
	let mut buf = [0; 1024];

    // In a loop, read data from the socket and write the data back.
    loop {
    	debug_info!(" ");
    	let n = match rsock.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => {
            	eprintln!("failed to read from socket; err = {:?}", e);
            	return Err(Box::new(e));
            }
        };

        debug_buffer_trace!(buf.as_ptr(),n,"receive buffer");
        let sbuf = Arc::new(Mutex::new(buf[0..n].to_vec()));
        tx.send((sbuf,uidx));
    }
    return Ok(());
}


async fn split_sock_listen(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let fmtstr :String;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{SplitSockError,"need at least port"}
	}
	fmtstr = format!("0.0.0.0:{}",sarr[0]);
	let (tx,rx) = tokio::sync::mpsc::unbounded_channel::<(Arc<Mutex<Vec<u8>>>,u64)>();
	let mut sockhdl :SockHandle = SockHandle::new(rx)?;
	let mut gidx :u64 = 0;

	let listener = TcpListener::bind(&fmtstr).await?;
	loop {
		debug_info!(" ");
		let (mut socket, _) = listener.accept().await?;
		let (mut rsock,mut wsock) = socket.into_split();
		debug_info!(" ");
		let ntx = tx.clone();
		gidx += 1;
		if gidx == 0 {
			gidx += 1;
		}
		let nidx = gidx;
		let (ctx,mut crx) = tokio::sync::mpsc::unbounded_channel::<Arc<Mutex<Vec<u8>>>>();
		let _ = sockhdl.add_snd(ctx,nidx)?;

		tokio::spawn(async move {
			tokio::select!{
				_ = rsock_handle(rsock,tx.clone(),nidx) => {},
				_ = wsock_handle(wsock,crx) => {},
			};
		});

	}

}

async fn split_sock_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
	let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
	let _ = init_exit_handle(sigv,tx.clone())?;

	tokio::select!{
		_val = ctrl_recv(&mut rx) => {

		},
		bval = split_sock_listen(ns) => {
			if bval.is_err() {
				return Err(bval.err().unwrap());
			}
		}
	}
	Ok(())
}


fn splitsock_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	//let res :Result<(),Box<dyn Error>>;
	init_log(ns.clone())?;
	let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(split_sock_main(ns.clone()))?;
	return Ok(());
}

#[extargs_map_function(splitsock_handler)]
pub fn load_sock_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let commandline = r#"
	{
		"splitsock<splitsock_handler>##port to listen on port##" : {
			"$" : 1
		}
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	return Ok(());
}