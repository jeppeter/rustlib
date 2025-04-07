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
use tokio::net::{UdpSocket};
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::sync::Mutex as AsyncMutex;
//use extutils::strop::{parse_u64};
use crate::asynfd::async_read_file;


extargs_error_class!{UdpSockError}


async fn udpsend_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let sarr = ns.get_array("subnargs");
	let mut remoteaddr :String = format!("127.0.0.1:7793");
	let mut localaddr :String = format!("0.0.0.0:9916");
	let udpsock : UdpSocket;
	let udpsize :usize = ns.get_int("udpsize") as usize;
	let input :String = ns.get_string("input");

	if sarr.len() > 0 {
		remoteaddr = format!("{}",sarr[0]);
	}

	if sarr.len() > 1 {
		localaddr = format!("{}",sarr[1]);
	}

	let ores = localaddr.parse::<std::net::SocketAddr>();
	if ores.is_err() {
		extargs_new_error!{UdpSockError,"parse [{}] error [{:?}]",localaddr,ores.err().unwrap()}
	}
	let laddr = ores.unwrap();
	let ores = std::net::UdpSocket::bind(&laddr);
	if ores.is_err() {
		extargs_new_error!{UdpSockError,"bind [{}] error [{:?}]",localaddr,ores.err().unwrap()}	
	}
	let std_sock = ores.unwrap();
	let ores = UdpSocket::from_std(std_sock);
	if ores.is_err() {
		extargs_new_error!{UdpSockError,"from_std [{}] error [{:?}]",localaddr,ores.err().unwrap()}	
	}
	udpsock = ores.unwrap();

	let ores = remoteaddr.parse::<std::net::SocketAddr>();
	if ores.is_err() {
		extargs_new_error!{UdpSockError,"parse [{}] error [{:?}]",remoteaddr,ores.err().unwrap()}
	}

	let raddr = ores.unwrap();
	udpsock.connect(&raddr).await?;

	let content = async_read_file(&input).await?;
	let mut wsize :usize = 0;

	let mut rbuf :Vec<u8>;
	rbuf = Vec::with_capacity(2048);
	while wsize < content.len(){
		let mut cursize :usize = udpsize;
		if (cursize + wsize) > content.len() {
			cursize = content.len() - wsize;
		}

		debug_trace!("w [{}..{}]",wsize,wsize + cursize);
		let wlen = udpsock.send(&content[wsize..(wsize + cursize)]).await?;
		wsize += wlen;
		debug_trace!("send {}",wsize);
		rbuf.fill(0);

		let rlen = udpsock.recv(&mut rbuf).await?;
		debug_trace!("rlen {}",rlen);
	}

	Ok(())
}


fn udpsend_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	//let res :Result<(),Box<dyn Error>>;
	init_log(ns.clone())?;
	let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(udpsend_main(ns.clone()))?;
	return Ok(());
}

async fn ctrl_recv(exitchl :&mut tokio::sync::mpsc::UnboundedReceiver<u32>) -> u32 {
	return exitchl.recv().await.unwrap();
}


#[allow(unreachable_code)]
async fn udp_recv_handler(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let mut localaddr :String = format!("0.0.0.0:7793");
	let sarr :Vec<String> = ns.get_array("subnargs");
	let udpsize :usize = ns.get_int("udpsize") as usize;

	if sarr.len() > 0 {
		localaddr = format!("{}",sarr[0]);
	}

	let ores = localaddr.parse::<std::net::SocketAddr>();
	if ores.is_err() {
		extargs_new_error!{UdpSockError,"parse [{}] error {:?}", localaddr,ores.err().unwrap()}
	}

	let laddr = ores.unwrap();
	let udpsock :UdpSocket = UdpSocket::bind(&laddr).await?;
	debug_trace!("listen on {} udpsize {}",localaddr,udpsize);
	loop {
		let mut rbuf :Vec<u8> = Vec::with_capacity(2048) ;
		rbuf.fill(0);
		let ores  = udpsock.recv_from(&mut rbuf).await;
		if ores.is_err() {
			debug_error!("error {:?}",ores.err().unwrap());
			continue;
		}

		let (rsize,raddr) = ores.unwrap();
		debug_trace!("rsize {}",rsize);
		udpsock.send_to(&rbuf[0..rsize],&raddr).await?;
	}

	Ok(())

}

async fn udprecv_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
	let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
	let _ = init_exit_handle(sigv,tx.clone())?;

	tokio::select!{
		_val = ctrl_recv(&mut rx) => {
			debug_trace!("ctrl_recv");
		},
		bval = udp_recv_handler(ns) => {
			if bval.is_err() {
				return Err(bval.err().unwrap());
			}
		}
	}
	Ok(())
}


fn udprecv_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	//let res :Result<(),Box<dyn Error>>;
	init_log(ns.clone())?;
	let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(udprecv_main(ns.clone()))?;
	return Ok(());
}



#[extargs_map_function(udpsend_handler,udprecv_handler)]
pub fn load_udp_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let commandline = r#"
	{
		"udpsize##to set udp size default 1250##" : 1250,
		"udpsend<udpsend_handler>##ip:port [localip:port] to send from input default 127.0.0.1:7793 default local :9916##" : {
			"$" : "*"
		},
		"udprecv<udprecv_handler>##:port to listen on udp default :7793##" : {
			"$" : "?"
		}
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	return Ok(());
}