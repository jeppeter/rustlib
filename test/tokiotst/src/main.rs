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
use extargsparse_worker::const_value::{COMMAND_SET,SUB_COMMAND_JSON_SET,COMMAND_JSON_SET,ENVIRONMENT_SET,ENV_SUB_COMMAND_JSON_SET,ENV_COMMAND_JSON_SET,DEFAULT_SET};

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::error::Error;
use std::boxed::Box;


#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

use futures::executor::block_on;

#[cfg(windows)]
mod wchar_windows;
#[cfg(windows)]
mod loglib_windows;
mod loglib;

#[allow(unused_imports)]
use loglib::{log_get_timestamp,log_output_function,init_log};


use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

extargs_error_class!{TokioRunError}

async fn tokio_listen_main(ns :NameSpaceEx) ->  Result<(), Box<dyn std::error::Error>>  {
	let sarr :Vec<String>;
	let fmtstr :String;
	sarr = ns.get_array("subnargs");
	if sarr.len() < 1 {
		extargs_new_error!{TokioRunError,"need at least port"}
	}
	if sarr.len() == 1 {
		fmtstr = format!("0.0.0.0:{}",sarr[0]);
	} else {
		fmtstr = format!("{}:{}", sarr[1],sarr[0]);
	}

	let listener = TcpListener::bind(&fmtstr).await?;
	loop {
		let (mut socket, _) = listener.accept().await?;
		tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
		});
	}
}

fn tokiolisten_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {	

	let res :Result<(),Box<dyn Error>>;
	init_log(ns.clone())?;
	res = block_on(tokio_listen_main(ns.clone()));
	return res;
}

#[extargs_map_function(tokiolisten_handler)]
fn main() -> Result<(),Box<dyn Error>> {
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	let commandline = r#"
	{
		"output|o" : null,
		"input|i" : null,
		"tokiolisten<tokiolisten_handler>##port [ipaddr] to listen on tokio##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	loglib::prepare_log(parser.clone())?;
	let ores = parser.parse_commandline_ex(None,None,None,None);
	if ores.is_err() {
		let e = ores.err().unwrap();
		eprintln!("{:?}", e);
		return Err(e);
	}
	return Ok(());
}
