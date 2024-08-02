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
#[allow(unused_imports)]
use extlog::{debug_info,debug_trace,debug_error,debug_buffer_trace,format_buffer_log,format_str_log};

use crate::exithdl_consts::{SIG_TERM,SIG_INT};
use crate::exithdl::{init_exit_handle};
use crate::logtrans::{init_log};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use extutils::strop::{parse_u64};


extargs_error_class!{SplitSockError}

struct SockHandleInner {
	rcv :tokio::sync::mpsc::UnboundedReceiver<(Arc<Mutex<Vec<u8>>>,u64)>,
	snds :Vec<tokio::sync::mpsc::UnboundedSender<Arc<Mutex<Vec<u8>>>>>,
	sndidx :Vec<u64>,
}

unsafe impl Send for SockHandleInner {}

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
		debug_trace!("nnxx");
		loop {
			debug_trace!("before rcv");
			let ores = self.rcv.recv().await;
			debug_trace!("receive_fn");
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
			{
				let cdata = data.lock().unwrap();
				debug_buffer_trace!(cdata.as_ptr(),cdata.len(),"inner receive and send");
			}
			
			let ores = snd.send(data);
			if ores.is_err() {
				debug_trace!("send error {:?}",ores.err().unwrap());
			}
		}
	}
}

#[derive(Clone)]
struct SockHandle {
	inner :Arc<UnsafeCell<SockHandleInner>>,
}

unsafe impl Send for SockHandle {}

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
		debug_trace!("before get inner");
		let s1 = unsafe {&mut *self.inner.get()};
		debug_trace!("receive_fn inner");
		let _ = s1.receive_fn().await;
		Ok(())
	}

}

async fn ctrl_recv(exitchl :&mut tokio::sync::mpsc::UnboundedReceiver<u32>) -> u32 {
	return exitchl.recv().await.unwrap();
}

#[allow(unreachable_code)]
async fn wsock_handle(mut wsock :tokio::net::tcp::OwnedWriteHalf,mut rx :tokio::sync::mpsc::UnboundedReceiver<Arc<Mutex<Vec<u8>>>>) -> Result<(),Box<dyn Error>> {
	let mut c2buf :Vec<u8> ;
	loop {
		let ores = rx.recv().await;
		if ores.is_none() {
			continue;
		}
		let wbuf = ores.unwrap();
		{
			let cbuf = wbuf.lock().unwrap();
			debug_buffer_trace!(cbuf.as_ptr(),cbuf.len(),"will send buffer");
			c2buf = vec![];
			let mut idx :usize = 0;
			while c2buf.len() < cbuf.len() {
				c2buf.push(cbuf[idx]);
				idx += 1;
			}

			{
				let mut csize :usize = 0;
				while csize < c2buf.len() {
					let _ = wsock.write_u8(c2buf[csize]);
					csize += 1;
				}

				//let ores = wsock.write_all(&c2buf).await;
				//if ores.is_err() {
				//	debug_error!("write error {:?}",ores.err().unwrap());
				//	continue;
				//}

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
    	debug_info!("will read");
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
        let ores = tx.send((sbuf,uidx));
        if ores.is_err() {
        	debug_error!("send error {:?}",ores.err().unwrap());
        }
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
	let mut bsock = sockhdl.clone();

	let listener = TcpListener::bind(&fmtstr).await?;
	tokio::spawn(async move {
		debug_trace!("new tokio spawn");
		let _ = bsock.receive_fn().await;
		debug_trace!("end tokio spawn");
	});

	loop {
		debug_info!("listen on {}",fmtstr);
		let (socket, _) = listener.accept().await?;
		let (rsock,wsock) = socket.into_split();
		debug_info!("split rsock and wsock");
		let ntx = tx.clone();
		gidx += 1;
		if gidx == 0 {
			gidx += 1;
		}
		let nidx = gidx;
		let (ctx,crx) = tokio::sync::mpsc::unbounded_channel::<Arc<Mutex<Vec<u8>>>>();
		let _ = sockhdl.add_snd(ctx,nidx)?;
		let mut csock = sockhdl.clone();

		tokio::spawn(async move {
			tokio::select!{
				_ = rsock_handle(rsock,ntx.clone(),nidx) => {},
				_ = wsock_handle(wsock,crx) => {},
			};
			let _ = csock.remove_snd(nidx);
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

#[allow(unreachable_code)]
async fn child_event_send(mut incv : tokio::sync::mpsc::UnboundedReceiver<String>,snd :tokio::sync::mpsc::UnboundedSender<String>) -> Result<(),Box<dyn Error>> {
	let mut curval :usize = 0;
	loop {

		let oval = incv.recv().await;
		if oval.is_none() {
			debug_trace!("none ");
			continue;
		}
		let val = oval.unwrap();
		debug_trace!("child receive [{}]",val);
		let nval = format!("child cnt[{}]",curval);
		let _ = snd.send(nval);
		curval += 1;
	}

	Ok(())
}


async fn main_event_send(mut incv : tokio::sync::mpsc::UnboundedReceiver<String>,snd :tokio::sync::mpsc::UnboundedSender<String>,cnt :usize) -> Result<(),Box<dyn Error>> {
	let mut curval :usize = 0;
	loop {
		if cnt != 0 && curval >= cnt {
			break;
		}
		let nstr = format!("main [{}]",curval);
		let _ = snd.send(nstr);

		let oval = incv.recv().await;
		if oval.is_none() {
			debug_trace!("none ");
			continue;
		}
		let val = oval.unwrap();
		debug_trace!("main receive [{}]",val);
		curval += 1;
	}

	Ok(())
}

async fn event_send_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String> = ns.get_array("subnargs");
	let mut cnt :usize = 10;
	if sarr.len() > 0 {
		cnt = parse_u64(&sarr[0])? as usize;
	}

	let (msnd,mrcv) = tokio::sync::mpsc::unbounded_channel::<String>();
	let (csnd,crcv) = tokio::sync::mpsc::unbounded_channel::<String>();

	tokio::select!{
		_ = main_event_send(mrcv,csnd,cnt) => {debug_trace!("main exit");},
		_ = child_event_send(crcv,msnd) => {debug_trace!("child exit");},
	}	
	Ok(())
}

fn evtsnd_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	//let res :Result<(),Box<dyn Error>>;
	init_log(ns.clone())?;
	let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(event_send_main(ns.clone()))?;
	return Ok(());
}


#[extargs_map_function(splitsock_handler,evtsnd_handler)]
pub fn load_sock_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let commandline = r#"
	{
		"splitsock<splitsock_handler>##port to listen on port##" : {
			"$" : 1
		},
		"evtsnd<evtsnd_handler>##[cnt] to send event call in cnt default 10##" : {
			"$" : "?"
		}
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	return Ok(());
}