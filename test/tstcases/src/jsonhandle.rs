#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};

#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::cell::RefCell;
use std::sync::Arc;
//use std::io::Write;
use std::error::Error;
use std::boxed::Box;
use std::rc::Rc;
use std::cell::UnsafeCell;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::fileop::{read_file_bytes,read_file,write_file_bytes};
#[allow(unused_imports)]
use super::strop::{parse_u64,decode_base64};
use std::any::Any;
use super::jsondata::{JSonPack,JSonUnpack};
use serde::{Deserialize, Serialize};



asn1obj_error_class!{JsonHdlError}

fn jpmergejup_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{JsonHdlError,"need merge jsonpack jsonunpack json file"}
    }
    let s = read_file(&sarr[0])?;
    let us = read_file(&sarr[1])?;
    let mut jp :JSonPack = JSonPack::new(&s)?;
    let jup :JSonUnpack = JSonUnpack::new(&us)?;
    {
        let mut refv :Vec<&str> = Vec::new();
        for s in sarr[2..].iter() {
            refv.push(s);
        }
        let _ = jp.merge_unpack_ref(&jup,&refv)?;
        let data = jp.pack()?;
        debug_buffer_trace!(data.as_ptr(),data.len(),"json pack ref");
    }
    let mut jp :JSonPack = JSonPack::new(&s)?;
    let jup :JSonUnpack = JSonUnpack::new(&us)?;
    
    let _ = jp.merge_unpack(&jup,&sarr[2..])?;
    let data = jp.pack()?;
    let output = ns.get_string("output");
    debug_buffer_trace!(data.as_ptr(),data.len(),"json pack");
    let _ = write_file_bytes(&output,&data)?;
    Ok(())
}

fn jupmergejp_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 3 {
        extargs_new_error!{JsonHdlError,"need merge jsonpack jsonunpack json file"}
    }
    let s = read_file(&sarr[0])?;
    let us = read_file(&sarr[1])?;
    let mut jup :JSonUnpack = JSonUnpack::new(&s)?;
    let jp :JSonPack = JSonPack::new(&us)?;
    let _ = jup.merge_pack(&jp,&sarr[2..])?;
    let data = jup.pack()?;
    let output = ns.get_string("output");
    debug_buffer_trace!(data.as_ptr(),data.len(),"json unpack");
    let _ = write_file_bytes(&output,&data)?;
    Ok(())
}

#[derive(Clone,Serialize,Deserialize)]
struct ExecCmd {
    #[serde(default = "cmds_default")]
    cmds :Vec<String>,
    #[serde(default = "vals_default")]
    vals :Vec<Vec<String>>,
}

impl Default for ExecCmd {
    fn default() -> Self {
        Self {
            cmds : Vec::new(),
            vals : Vec::new(),
        }
    }
}

fn cmds_default() -> Vec<String> {
    return Vec::new();
}

fn vals_default() -> Vec<Vec<String>> {
    return Vec::new();
}

#[derive(Clone)]
enum FuncCall {
    CallFunc(Rc<dyn Fn(&str,&[String]) -> Result<(),Box<dyn Error>>>),
}


struct ExecCmdHandlerInner {
    fname :String,
    cmd : ExecCmd,
    runcmds : Rc<RefCell<HashMap<String,Rc<RefCell<FuncCall>>>>>,
}

#[derive(Clone)]
struct ExecCmdHandler {
    inner :Arc<UnsafeCell<ExecCmdHandlerInner>>,
}



impl ExecCmdHandlerInner {
    fn handle_exec(&mut self, cmd :&str, vals :&[String]) -> Result<(),Box<dyn Error>> {
        self.cmd.cmds.push(format!("{}",cmd));
        let mut insertvals :Vec<String> = Vec::new();
        let mut idx :usize = 0;
        debug_trace!("run _handle_exec");
        while idx < vals.len() {
            insertvals.push(format!("{}",vals[idx]));
            idx += 1;
        }
        self.cmd.vals.push(insertvals);
        debug_trace!("cmd [{:?}] vals [{:?}] self {:p}",self.cmd.cmds,self.cmd.vals,self);
        Ok(())
    }

    fn handle_run(&mut self,cmd :&str, vals :&[String]) -> Result<(),Box<dyn Error>>{
        self.cmd.cmds.push(format!("{}",cmd));
        let mut insertvals :Vec<String> = Vec::new();
        let mut idx :usize = 0;
        debug_trace!("run _handle_run");
        while idx < vals.len() {
            insertvals.push(format!("{}",vals[idx]));
            idx += 1;
        }
        self.cmd.vals.push(insertvals);
        debug_trace!("cmd [{:?}] vals [{:?}] self {:p}",self.cmd.cmds,self.cmd.vals,self);
        Ok(())
    }

    fn insert_funcs(&mut self, parent :ExecCmdHandler) -> Result<(),Box<dyn Error>> {
        let b = Arc::new(UnsafeCell::new(parent));
        let mut bmut = self.runcmds.borrow_mut();
        let s1 = b.clone();
        bmut.insert(format!("run"),Rc::new(RefCell::new(FuncCall::CallFunc(Rc::new(move |k,v| {let  c :&mut ExecCmdHandler = unsafe {&mut *s1.get()};
            c.handle_run(k,v)
        } )))));
        let s1 = b.clone();
        bmut.insert(format!("exec"),Rc::new(RefCell::new(FuncCall::CallFunc(Rc::new(move |k,v| {let  c :&mut ExecCmdHandler = unsafe {&mut *s1.get()};
            c.handle_exec(k,v)
        } )))));
        Ok(())
    }

    fn new(fname :&str) -> Result<Self,Box<dyn Error>> {
        let mut retv :Self = Self {
            fname : format!("{}",fname),
            cmd : ExecCmd::default(),
            runcmds : Rc::new(RefCell::new(HashMap::new())),
        };
        let s = read_file(fname)?;
        retv.cmd = serde_json::from_str(&s)?;
        Ok(retv)
    }

    fn call_funcs(&mut self,cmd :&str , vals :&[String]) -> Result<(),Box<dyn Error>> {
        match self.runcmds.borrow().get(cmd) {
            Some(f1) => {
                let f2 :&FuncCall = &f1.borrow();
                match f2 {
                    FuncCall::CallFunc(f) => {
                        return f(cmd,vals);
                    },
                }
            },
            None => {
                extargs_new_error!{JsonHdlError,"get function [{}] error",cmd}
            }
        }
    }

    fn flush_file(&self) -> Result<(),Box<dyn Error>> {
        let s = serde_json::to_string(&self.cmd)?;
        write_file_bytes(&self.fname,s.as_bytes())?;
        debug_trace!("self {:p}",self);
        Ok(())
    }

}

impl ExecCmdHandler {
    fn handle_exec(&mut self, cmd :&str, vals :&[String]) -> Result<(),Box<dyn Error>> {
        let s1 :&mut ExecCmdHandlerInner = unsafe {&mut *self.inner.get()};
        return s1.handle_exec(cmd,vals);
    }

    fn handle_run(&mut self,cmd :&str, vals :&[String]) -> Result<(),Box<dyn Error>>{
        let s1 :&mut ExecCmdHandlerInner = unsafe {&mut *self.inner.get()};
        return s1.handle_run(cmd,vals);
    }


    fn new(fname :&str) -> Result<Self,Box<dyn Error>> {
        let retv :Self = Self {
            inner : Arc::new(UnsafeCell::new(ExecCmdHandlerInner::new(fname)?)),
        };
        let s1 :&mut ExecCmdHandlerInner = unsafe {&mut *retv.inner.get()};
        s1.insert_funcs(retv.clone())?;
        Ok(retv)
    }

    fn call_funcs(&mut self,cmd :&str , vals :&[String]) -> Result<(),Box<dyn Error>> {
        let s1 :&mut ExecCmdHandlerInner = unsafe {&mut *self.inner.get()};
        return s1.call_funcs(cmd,vals);        
    }

    fn flush_file(&self) -> Result<(),Box<dyn Error>> {
        let s1 :&mut ExecCmdHandlerInner = unsafe {&mut *self.inner.get()};
        return s1.flush_file();
    }

}


fn callfunc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    let cmd :String;
    let mut vals :Vec<String> = Vec::new();
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 1 {
        extargs_new_error!{JsonHdlError,"need cmd"}
    }
    let fname = ns.get_string("input");
    let mut cv = ExecCmdHandler::new(&fname)?;
    cmd = format!("{}",sarr[0]);
    let mut idx :usize = 1;
    while idx < sarr.len() {
        vals.push(format!("{}",sarr[idx]));
        idx += 1;
    }
    cv.call_funcs(&cmd,&vals)?;
    cv.flush_file()?;
    Ok(())
}


#[extargs_map_function(jpmergejup_handler,jupmergejp_handler,callfunc_handler)]
pub fn load_json_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
        "jpmergejup<jpmergejup_handler>##from files to input and to output##" : {
            "$" : "+"
        },
        "jupmergejp<jupmergejp_handler>##from files to input and to output##" : {
            "$" : "+"
        },
        "callfunc<callfunc_handler>##cmd vals ... to call funcs##" : {
            "$" : "+"
        }
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}