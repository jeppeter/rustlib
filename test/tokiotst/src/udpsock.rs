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
use extutils::strop::{parse_u64,parse_computer_size_i64};
use extutils::timeop::{get_cur_ticks};
use crate::asynfd::async_read_file;


extargs_error_class!{UdpSockError}


async fn udp_send_send(_sock :&tokio::net::UdpSocket,data :&[u8],udpsize :usize,sleepwhile :u32,waitsize :usize) -> Result<(),Box<dyn Error>>{
    let mut widx :usize = 0;
    let mut lastidx :usize = 0;
    while widx < data.len() {
        let mut cursize = udpsize;
        if (cursize + widx) > data.len() {
            cursize = data.len() - widx;
        }
        let n = _sock.send(&data[widx..(widx+cursize)]).await?;
        widx += n;
        //debug_trace!("send {} widx {} lastidx {}",n, widx, lastidx);
        if (widx - lastidx) > waitsize {
            /*to wait for a while*/
            //debug_trace!("sleep a while");
            let _ = tokio::time::sleep(tokio::time::Duration::from_millis(sleepwhile as u64)).await;
            lastidx = widx;
            //debug_trace!("sleep back {}",lastidx);
        }
    }
    Ok(())
}


async fn udp_send_recv(_sock :&tokio::net::UdpSocket,data :&[u8]) -> Result<(),Box<dyn Error>> {
    let mut idx :usize=0;
    let mut rbuf :Vec<u8> = [0;4096].to_vec();
    while idx < data.len() {
        let n = _sock.recv(&mut rbuf).await?;
        let mut errcnt :usize = 0;
        for i in 0..n {
            if rbuf[i] != data[idx+i] {
                //debug_error!("0x{:02x} 0x{:02x} != 0x{:02x}", idx +i, rbuf[i],data[idx+i]);
                errcnt += 1;
            }
        }

        if errcnt != 0 {
            debug_error!("on 0x{:02x} errcnt {}", idx, errcnt);
        }

        //debug_trace!("recv {}",n);
        idx += n;
    }

    Ok(())
}


async fn udpsend_inner(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let sarr = ns.get_array("subnargs");
    let mut remoteaddr :String = format!("127.0.0.1:7793");
    let mut localaddr :String = format!("0.0.0.0:0");
    let udpsock : UdpSocket;
    let udpsize :usize = ns.get_int("udpsize") as usize;
    let input :String = ns.get_string("input");
    let waitsize :usize = parse_computer_size_i64(&(ns.get_string("waitsize")))? as usize;

    let udpwhile = ns.get_int("udpwhile") as u32;

    if sarr.len() > 0 {
        remoteaddr = format!("{}",sarr[0]);
    }

    if sarr.len() > 1 {
        localaddr = format!("{}",sarr[1]);
    }
    debug_trace!("sarr {:?}", sarr);

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
    let _ = std_sock.set_nonblocking(true)?;
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
    let sticks = get_cur_ticks();
    let content = async_read_file(&input).await?;
    let (_r1,_r2) = tokio::join!(
        udp_send_recv(&udpsock,&content),
        udp_send_send(&udpsock,&content,udpsize,udpwhile,waitsize)
        );

    let eticks = get_cur_ticks();
    debug_trace!("elapse {}", eticks - sticks);

    Ok(())
}

async fn udpsend_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
    let _ = init_exit_handle(sigv,tx.clone())?;

    tokio::select!{
        _val = ctrl_recv(&mut rx,ns.clone()) => {
            debug_trace!("thread {:?} ctrl_recv",std::thread::current().id());
        },
        bval = udpsend_inner(ns) => {
            if bval.is_err() {
                return Err(bval.err().unwrap());
            }
        }
    }
    Ok(())
}


fn udpsend_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> { 

    //let res :Result<(),Box<dyn Error>>;
    init_log(ns.clone())?;
    let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(udpsend_main(ns.clone()))?;
    return Ok(());
}

async fn ctrl_recv(exitchl :&mut tokio::sync::mpsc::UnboundedReceiver<u32>, ns:NameSpaceEx) -> u32 {
    let mut idx :u32 = 0;
    let maxcnt :u32 = ns.get_int("ctrlcnt") as u32;
    loop {
        let retv = exitchl.recv().await.unwrap();
        if idx >= maxcnt {
            return retv;
        }
        debug_trace!("thread {:?} wait cnt {}",std::thread::current().id(),idx);
        idx += 1;
    } 
}


async fn udp_recv_send(rx :&mut tokio::sync::mpsc::UnboundedReceiver<(Vec<u8>,std::net::SocketAddr)>,_sock :&tokio::net::UdpSocket) {
    loop {
        //debug_trace!("thread {:?} will receive send",std::thread::current().id());
        let ores = rx.recv().await;
        if ores.is_none() {
            break;
        }
        let (rbuf,_raddr) = ores.unwrap();
        if rbuf.len() == 0 {
            break;
        }
        //debug_trace!("receive {} addr {:?}", rbuf.len(),_raddr);

        let _ = _sock.send_to(&rbuf,&_raddr).await; 
        
    }
    return;
}

#[allow(unreachable_code)]
async fn udp_recv_recv(tx :&tokio::sync::mpsc::UnboundedSender<(Vec<u8>,std::net::SocketAddr)>,_sock :&tokio::net::UdpSocket) {
    let mut rbuf :Vec<u8> = [0; 4096].to_vec();
    let mut rsize :usize;
    loop {
        let cbuf :Vec<u8>;
        let raddr :std::net::SocketAddr;
        rbuf.fill(0);
        //debug_trace!("thread {:?} will receive",std::thread::current().id());
        let ores  = _sock.recv_from(&mut rbuf).await;
        if ores.is_err() {
            debug_error!("error {:?}",ores.err().unwrap());
            continue;
        }

        (rsize,raddr) = ores.unwrap();

        //debug_trace!("rsize {}",rsize);
        cbuf  = rbuf[0..rsize].to_vec().clone();
        //debug_trace!("send buffer {}",cbuf.len());

        let ores = tx.send((cbuf,raddr));
        if ores.is_err() {
            debug_error!("send error {:?}", ores.err().unwrap());
            continue;
        }
        //debug_trace!("tx send ok");
    }
    return;
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
    let basesock :std::net::UdpSocket = std::net::UdpSocket::bind(&laddr)?;
    let _ = basesock.set_nonblocking(true)?;
    //let ncstd = basesock.try_clone()?;
    let udpsock :UdpSocket = UdpSocket::from_std(basesock)?;
    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<(Vec<u8>,std::net::SocketAddr)>();
    debug_trace!("listen on {} udpsize {}",localaddr,udpsize);
    let mut rbuf :Vec<u8>;
    rbuf = vec![];
    for _ in 0..2048 {
        rbuf.push(0);
    }


    tokio::select!{
        _ = udp_recv_recv(&tx,&udpsock) => {
            debug_trace!("udp recv recv");
        },
        _= udp_recv_send(&mut rx,&udpsock) => {
            debug_trace!("udp recv send");
        }
    }



    Ok(())

}

async fn udprecv_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
    let _ = init_exit_handle(sigv,tx.clone())?;

    tokio::select!{
        _val = ctrl_recv(&mut rx,ns.clone()) => {
            debug_trace!("thread {:?} ctrl_recv",std::thread::current().id());
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

#[allow(unreachable_code)]
async fn simplechl_sender(tx :&tokio::sync::mpsc::UnboundedSender<(Vec<u8>,u32)>,timeout :u64) -> Result<(),Box<dyn Error>> {
    let  mut cnt :u32 = 0;
    loop {
        let _ = tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
        let _ = tx.send((vec![],cnt));
        debug_trace!("send {}",cnt);
        cnt += 1;
    }
    Ok(())
}

async fn simplechl_receiver(rx :&mut tokio::sync::mpsc::UnboundedReceiver<(Vec<u8>,u32)>) -> Result<(),Box<dyn Error>> {
    loop {
        let ores = rx.recv().await;
        if ores.is_none() {
            break;
        }
        let (_,val) = ores.unwrap();
        debug_trace!("receive {}", val);
    }
    Ok(())
}

async fn async_simple_chl_handler(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<(Vec<u8>,u32)>();
    let mut timeout :u64 = 1000;
    let sarr = ns.get_array("subnargs");
    if sarr.len() > 0 {
        timeout = parse_u64(&sarr[0])? as u64;
    }

    tokio::select!{
        _= simplechl_receiver(&mut rx) => {
            debug_trace!("receive end");
        },
        _ = simplechl_sender(&tx,timeout) => {
            debug_trace!("send end");
        },
    }
    Ok(())

}

async fn simplechl_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
    let _ = init_exit_handle(sigv,tx.clone())?;

    tokio::select!{
        _val = ctrl_recv(&mut rx,ns.clone()) => {
            debug_trace!("ctrl_recv");
        },
        bval = async_simple_chl_handler(ns) => {
            if bval.is_err() {
                return Err(bval.err().unwrap());
            }
        }
    }
    Ok(())
}



fn simplechl_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   

    //let res :Result<(),Box<dyn Error>>;
    init_log(ns.clone())?;
    let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(simplechl_main(ns.clone()))?;
    return Ok(());
}

async fn timeout_main(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
    let sigv :Vec<u32> = vec![SIG_TERM,SIG_INT];
    let (tx,mut _rx) = tokio::sync::mpsc::unbounded_channel::<u32>();
    let _ = init_exit_handle(sigv,tx.clone())?;
    let mut ntime :u64 = 500;

    let sarr = ns.get_array("subnargs");
    if sarr.len() > 0 {
        ntime = parse_u64(&sarr[0])?;
    }

    let ores = tokio::time::timeout(tokio::time::Duration::from_millis(ntime), async {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        Ok::<(),Box<dyn Error>>(())
    }).await;

    if ores.is_err() {
        println!("timeout");
    } else {
        println!("Ok");
    }

    Ok(())
}

fn timeout_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   

    //let res :Result<(),Box<dyn Error>>;
    init_log(ns.clone())?;
    let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(timeout_main(ns.clone()))?;
    return Ok(());
}


#[extargs_map_function(udpsend_handler,udprecv_handler,simplechl_handler,timeout_handler)]
pub fn load_udp_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let commandline = r#"
    {
        "ctrlcnt" : 0,
        "udpsize##to set udp size default 1250##" : 1250,
        "udpwhile" : 10,
        "waitsize" : "60k",
        "udpsend<udpsend_handler>##ip:port [localip:port] to send from input default 127.0.0.1:7793 default local 0.0.0.0:0##" : {
            "$" : "*"
        },
        "udprecv<udprecv_handler>##:port to listen on udp default 0.0.0.0:7793##" : {
            "$" : "?"
        },
        "simplechl<simplechl_handler>##[timeout] default timeout 1 second##" : {
            "$" : "?"
        },
        "timehdl<timeout_handler>##[timeout] to call tokio::time::timeout functions##" : {
            "$" : "?"
        }
    }
    "#;
    extargs_load_commandline!(parser,commandline)?;
    return Ok(());
}