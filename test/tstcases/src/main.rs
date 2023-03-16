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
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[cfg(windows)]
mod wchar_windows;
#[cfg(windows)]
mod loglib_windows;
mod loglib;
mod strop;
mod fileop;
mod filehdl;
mod reglib;
mod reghdl;
mod logtst;
mod credlib;
mod credhdl;
mod pelib;
mod timeop;
mod pehdl;
mod ssllib;
mod sslhdl;
mod pemlib;
mod gpglib;
mod rsalib;
mod cryptlib;
mod gpghdl;
mod crypthdl;
mod asn1def;
mod asn1tst;
mod pkcs7;
mod ossllib;
mod osslhdl;
mod ecchdl;


#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
	let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
	let commandline = r#"
	{
		"output|o" : null,
		"input|i" : null
	}
	"#;
	extargs_load_commandline!(parser,commandline)?;
	reghdl::load_reg_handler(parser.clone())?;
	loglib::prepare_log(parser.clone())?;
	logtst::load_log_handler(parser.clone())?;
	credhdl::load_cred_handler(parser.clone())?;
	pehdl::load_pe_handler(parser.clone())?;
	filehdl::load_file_handler(parser.clone())?;
	sslhdl::load_ssl_handler(parser.clone())?;
	asn1tst::load_asn1_handler(parser.clone())?;
	rsalib::load_rsa_handler(parser.clone())?;
	crypthdl::load_crypto_handler(parser.clone())?;
	pkcs7::load_pkcs7_handler(parser.clone())?;
	gpghdl::load_gpg_handler(parser.clone())?;
	osslhdl::load_ossl_handler(parser.clone())?;
	ecchdl::load_ecc_handler(parser.clone())?;
	let ores = parser.parse_commandline_ex(None,None,None,None);
	if ores.is_err() {
		let e = ores.err().unwrap();
		eprintln!("{:?}", e);
		return Err(e);
	}
	return Ok(());
}
