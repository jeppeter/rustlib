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
use extargsparse_worker::key::{KEYWORD_SUBCOMMAND};


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
use extutils::logtrans::{prepare_log};



mod dlopentst;


extargs_error_class!{RsExecError}

#[extargs_map_function()]
fn main() -> Result<(),Box<dyn Error>> {
    let parser :ExtArgsParser = ExtArgsParser::new(None,None)?;
    let commandline = format!(r#"
    {{

        "output|o" : null,
        "input|i" : null
    }}
    "#);
    extargs_load_commandline!(parser,&commandline)?;
    prepare_log(parser.clone())?;
    dlopentst::load_dlopen_handler(parser.clone())?;
    let ores = parser.parse_commandline_ex(None,None,None,None);
    if ores.is_err() {
        let e = ores.err().unwrap();
        eprintln!("{:?}", e);
        return Err(e);
    }
    let ns :NameSpaceEx = ores.unwrap();
    if ns.get_string(KEYWORD_SUBCOMMAND).len() == 0 {
        eprintln!("no subcommand match");
        extargs_new_error!{RsExecError,"no subcommand match"}
    }

    drop(ns);
    drop(parser);

    return Ok(());
}
