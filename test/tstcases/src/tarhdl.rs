use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
//use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_worker::namespace::{NameSpaceEx};
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};


#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::*;
use tar::{Builder,Archive};
use std::io::Write;

use chrono::{NaiveDateTime};

extargs_error_class!{TarHdlError}

struct FileOutput {
    out :Vec<std::io::Stdout>,
    file :Vec<std::fs::File>,
}

impl FileOutput {
    fn new(n :&str) -> Result<Self,Box<dyn Error>> {
        let  mut retv :Self = Self {
            out : vec![],
            file :vec![],
        };

        if n.len() == 0 {
            retv.out.push(std::io::stdout());
        } else {
            retv.file.push(std::fs::File::create(n)?);
        }
        Ok(retv)
    }
}

impl Write for FileOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.out.len() > 0 {
            return self.out[0].write(buf);
        } else if self.file.len() > 0 {
            return self.file[0].write(buf);
        }
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"no specified"));
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if self.out.len() > 0 {
            return self.out[0].flush();
        } else if self.file.len() > 0 {
            return self.file[0].flush();
        }
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"no specified"));
    }
}

struct FileInput {
    inv :Vec<std::io::Stdin>,
    file :Vec<std::fs::File>,
}

impl FileInput {
    fn new(n :&str) -> Result<Self,Box<dyn Error>> {
        let  mut retv :Self = Self {
            inv : vec![],
            file :vec![],
        };

        if n.len() == 0 {
            retv.inv.push(std::io::stdin());
        } else {
            retv.file.push(std::fs::File::open(n)?);
        }
        Ok(retv)
    }
}

impl std::io::Read for FileInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.inv.len() > 0 {
            return self.inv[0].read(buf);
        } else if self.file.len() > 0 {
            return self.file[0].read(buf);
        }
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"no specified"));
    }
}

struct ReadData {
    data:Vec<u8>,
    curidx :usize,
}

impl ReadData {
    fn new() -> Self {
        Self {
            data :vec![],
            curidx : 0,
        }
    }
}

impl Write for ReadData {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend(buf);
        return Ok(buf.len());
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Read for ReadData {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.curidx >= self.data.len() {
            return Ok(0);
        }

        let mut size :usize = 0;
        while size < buf.len() && self.curidx < self.data.len() {
            buf[size] = self.data[self.curidx];
            self.curidx += 1;
            size += 1;
        }

        Ok(size)
    }

}



fn tarcreate_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   
    let sarr :Vec<String>  = ns.get_array("subnargs");
    let output :String = ns.get_string("output");
    let mut tarout :tar::Builder<FileOutput>;

    init_log(ns.clone())?;
    if sarr.len() < 1 {
        extargs_new_error!{TarHdlError,"need file ..."}
    }

    tarout = Builder::new(FileOutput::new(&output)?);


    for f in sarr.iter() {
        let narr :Vec<&str> = f.split(":").collect();
        if narr.len() <= 1 {
            tarout.append_path(narr[0])?;
        } else {
            let mut infile :std::fs::File = std::fs::File::open(narr[0])?;
            tarout.append_file(narr[1],&mut infile)?;
        }
    }

    tarout.finish()?;
    Ok(())
}

fn tardelete_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   
    let sarr :Vec<String>  = ns.get_array("subnargs");
    let output :String = ns.get_string("output");
    let input :String = ns.get_string("input");
    let mut tarout :tar::Builder<FileOutput>;
    let mut tarin :tar::Archive<FileInput>;
    let entries :tar::Entries<FileInput>;

    init_log(ns.clone())?;
    if sarr.len() < 1 {
        extargs_new_error!{TarHdlError,"need file ..."}
    }

    tarin = Archive::new(FileInput::new(&input)?);
    tarout = Builder::new(FileOutput::new(&output)?);
    entries = tarin.entries()?;
    for file in entries {
        let mut matched :bool = false;
        if file.is_ok() {
            let mut f = file.unwrap();
            let opath = f.path();
            if opath.is_ok() {
                let name = format!("{}",opath.unwrap().display());
                for n in sarr.iter() {
                    if name == *n {
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                let header = f.header().clone();
                let mut rd :ReadData = ReadData::new();
                std::io::copy(&mut f,&mut rd)?;
                tarout.append(&header,&mut rd)?;
            }
        }
    }

    tarout.finish()?;
    Ok(())
}

fn taradd_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>  = ns.get_array("subnargs");
    let output :String = ns.get_string("output");
    let input :String = ns.get_string("input");
    let mut tarout :tar::Builder<FileOutput>;
    let mut tarin :tar::Archive<FileInput>;
    let entries :tar::Entries<FileInput>;

    init_log(ns.clone())?;
    if sarr.len() < 1 {
        extargs_new_error!{TarHdlError,"need file ..."}
    }

    tarin = Archive::new(FileInput::new(&input)?);
    tarout = Builder::new(FileOutput::new(&output)?);
    entries = tarin.entries()?;
    for file in entries {
        if file.is_ok() {
            let mut f = file.unwrap();

            let header = f.header().clone();
            let mut rd :ReadData = ReadData::new();
            std::io::copy(&mut f,&mut rd)?;
            tarout.append(&header,&mut rd)?;
        }
    }

    for f in sarr.iter() {
        let narr :Vec<&str> = f.split(":").collect();
        if narr.len() <= 1 {
            tarout.append_path(narr[0])?;
        } else {
            let mut infile :std::fs::File = std::fs::File::open(narr[0])?;
            tarout.append_file(narr[1],&mut infile)?;
        }
    }


    tarout.finish()?;
    Ok(())
}

fn timeget_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>  = ns.get_array("subnargs");
    let format:&str = "%Y-%m-%d %H:%M:%S";

    init_log(ns.clone())?;

    for f in sarr.iter() {
        let ct :NaiveDateTime = NaiveDateTime::parse_from_str(f,format)?;
        println!("{} parse date {}", f,ct);
    }
    Ok(())
}


#[extargs_map_function(tarcreate_handler,tardelete_handler,taradd_handler,timeget_handler)]
pub fn load_tar_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
        "maxtarfiles" : 10,
        "tarcreate<tarcreate_handler>##files... to create tar to output##" : {
            "$" : "+"
        },
        "tardel<tardelete_handler>##fname ... to delete file##" : {
            "$" : "+"
        },
        "taradd<taradd_handler>##fname ... to add file##" : {
            "$" : "+"
        },
        "timeget<timeget_handler>##timestr ... to transformat yyyy-mm-dd HH:MM:SS##" : {
            "$" : "+"
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}