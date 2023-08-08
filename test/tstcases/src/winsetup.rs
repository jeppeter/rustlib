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
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log,debug_error};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

use winapi::shared::guiddef::{GUID};
//use winapi::um::setupapi::{SetupDiGetClassDevsW,SetupDiDestroyDeviceInfoList,HDEVINFO,DIGCF_ALLCLASSES,DIGCF_DEVICEINTERFACE,DIGCF_PRESENT};
use winapi::um::setupapi::*;
use winapi::um::cfgmgr32::*;
use winapi::um::handleapi::{INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::{LoadLibraryA,FreeLibrary,GetProcAddress};
use winapi::shared::minwindef::{DWORD,BOOL,ULONG,HMODULE};
use winapi::shared::ntdef::{UNICODE_STRING,HANDLE,NTSTATUS,CHAR};
use winapi::shared::devpropdef::*;
use winapi::um::errhandlingapi::{GetLastError};
use std::ptr::{null_mut,null};
//use libc::{malloc,free,size_t,c_void};
use libc::{c_void,malloc,free};
use crate::strop::{parse_u64};
//use crate::fileop::{write_file_bytes};
use crate::automem::*;
use crate::wchar_windows::wstr_to_str;
//use std::io::Write;


extargs_error_class!{WinSetupError}



#[derive(Clone)]
struct HwProp {
    guid :String,
    propidx :u32,
    buf :Vec<u8>,
}

impl HwProp {
    pub fn new(guid :&str , propidx :u32 , buf :&[u8]) -> Self {
        Self {
            guid :guid.to_string(),
            propidx : propidx,
            buf :buf.to_vec(),
        }
    }
}

#[derive(Clone)]
struct HwInfo {
    props :Vec<HwProp>,
}

impl HwInfo {
    #[allow(dead_code)]
    pub fn get_prop(&self,guidstr :&str, propidx :u32) -> Option<HwProp> {
        let mut retv :Option<HwProp> = None;
        for p in self.props.iter() {
            if p.guid == guidstr && propidx == p.propidx {
                retv = Some(p.clone());
                break;
            }
        }
        return retv;
    }
}

macro_rules! GET_HEX_VAL {
    ( $tv:expr, $ch :expr) => {
        $tv <<= 4;
        if $ch >= '0' as u8 && $ch <= '9' as u8 {
            $tv += ($ch - ('0' as u8)) as u32;
        } else if $ch >= 'a' as u8 && $ch <= 'f' as u8 {
            $tv += ($ch - ('a' as u8) + 10) as u32;
        } else if $ch >= 'A' as u8 && $ch <= 'F' as u8 {
            $tv += ($ch - ('A' as u8) + 10) as u32;
        } else {
            extargs_new_error!{WinSetupError,"not valid char [{}]", $ch}
        }
    }
}

fn parse_guid(ins :&str) -> Result<GUID,Box<dyn Error>> {
    let mut retv :GUID = GUID::default();
    let bs = ins.as_bytes();
    let mut cv :u32;
    if bs[0] != ('{' as u8) {
        extargs_new_error!{WinSetupError,"bs[0] [{}] != {{", bs[0] as char}
    }

    cv = 0;
    for i in 1..9 {
        GET_HEX_VAL!(cv,bs[i]);
    }

    retv.Data1 = cv;
    if bs[9] != '-' as u8 {
        extargs_new_error!{WinSetupError,"bs[9] [{}] != -",bs[9] as char}
    }

    cv = 0;
    for i in 10..14 {
        GET_HEX_VAL!(cv,bs[i]);
    }

    retv.Data2 = cv as u16;

    if bs[14] != '-' as u8 {
        extargs_new_error!{WinSetupError,"bs[14] [{}] != -",bs[14] as char}
    }

    cv = 0;
    for i in 15..19 {
        GET_HEX_VAL!(cv,bs[i]);
    }

    retv.Data3 = cv as u16;

    if bs[19] != '-' as u8 {
        extargs_new_error!{WinSetupError,"bs[19] [{}] != -",bs[19] as char}
    }

    cv = 0;
    for i in 20..24 {
        GET_HEX_VAL!(cv,bs[i]);
        if (i % 2) != 0 {
            retv.Data4[((i- 20) / 2)] = cv as u8;
            cv = 0;         
        }
    }

    if bs[24] != '-' as u8 {
        extargs_new_error!{WinSetupError,"bs[24] [{}] != -",bs[24] as char}
    }

    cv = 0;
    for i in 25..37 {
        GET_HEX_VAL!(cv,bs[i]);
        if (i % 2) == 0 {
            retv.Data4[(i- 21)/2] = cv as u8;
            cv = 0;
        }
    }

    if bs[37] != '}' as u8 {
        extargs_new_error!{WinSetupError,"bs[37] [{}] != }}",bs[37] as char}
    }

    Ok(retv)
}

fn format_guid(pguid :&GUID) -> String {
    let mut rets :String = "{".to_string();
    let mut cv :u16 = 0;

    rets.push_str(&format!("{:08x}",pguid.Data1));
    rets.push_str("-");
    rets.push_str(&format!("{:04x}", pguid.Data2));
    rets.push_str("-");
    rets.push_str(&format!("{:04x}", pguid.Data3));

    cv += pguid.Data4[0] as u16;
    cv <<= 4;
    cv += pguid.Data4[1] as u16;
    rets.push_str(&format!("{:04x}", cv));
    rets.push_str("-");
    for i in 2..8 {
        rets.push_str(&format!("{:02x}",pguid.Data4[i]));
    }

    rets.push_str("}");
    rets
}

fn get_hw_props(pinfo :HDEVINFO,pndata :PSP_DEVINFO_DATA) -> Result<Vec<HwProp>,Box<dyn Error>> {
    let mut bret :BOOL;
    let mut requiresize :DWORD = 0;
    let mut props :Vec<HwProp> = Vec::new();
    let mut propguids :AutoMem<DEVPROPKEY> = AutoMem::new(0);
    let mut cfgret :CONFIGRET;
    let mut proptype :DEVPROPTYPE;
    let mut bufsize :ULONG = 0;
    let mut reqbufsize :ULONG;
    let mut pbuf :AutoMem<u8> = AutoMem::new(0);
    let mut cpropguids :*mut DEVPROPKEY;

    unsafe {
        bret = SetupDiGetDevicePropertyKeys(pinfo,pndata,null_mut(),0,&mut requiresize,0);    
    }
    
    if bret == 0 {
        debug_trace!("requiresize [{}]", requiresize);
        propguids.reset(requiresize as usize);
        let allocsize = requiresize;
        unsafe{
            bret = SetupDiGetDevicePropertyKeys(pinfo,pndata,propguids.ptr_mut(0),allocsize,&mut requiresize,0);    
        }        
        if bret == 0 {
            extargs_new_error!{WinSetupError,"can not get property"}
        }
    }

    for i in 0..requiresize {
        proptype = 0;
        reqbufsize = 0;
        cpropguids = propguids.ptr_mut(i as usize);
        unsafe{
            cfgret = CM_Get_DevNode_PropertyW((*pndata).DevInst,cpropguids,&mut proptype,null_mut(),&mut reqbufsize,0);    
        }
        
        if cfgret != CR_SUCCESS {
            if cfgret != CR_BUFFER_SMALL && cfgret != CR_NO_SUCH_VALUE {
                extargs_new_error!{WinSetupError,"get property error 0x{:x}",cfgret}
            }
            if cfgret == CR_NO_SUCH_VALUE {
                unsafe{
                    debug_error!("[{}].[0x:{:x}] no value", format_guid(&((*cpropguids).fmtid)), (*cpropguids).pid);    
                }                
                continue;
            }

            if reqbufsize > bufsize {
                pbuf.reset(reqbufsize as usize);
                bufsize = reqbufsize;    
            }           


            proptype = 0;
            reqbufsize = bufsize;
            cpropguids = propguids.ptr_mut(i as usize);
            unsafe {
                cfgret = CM_Get_DevNode_PropertyW((*pndata).DevInst,cpropguids,&mut proptype,pbuf.ptr_mut(0),&mut reqbufsize,0);
            }            
            if cfgret != CR_SUCCESS {
                extargs_new_error!{WinSetupError,"get property error 0x{:x}",cfgret}
            }

            /*now we should */
            cpropguids = propguids.ptr_mut(i as usize);
            let mut curprop :HwProp;
            let empvec :Vec<u8> = Vec::new();

            unsafe {
                curprop = HwProp::new(&format_guid(&((*cpropguids).fmtid)), (*cpropguids).pid,&empvec);
            }
            for j in 0..reqbufsize {
                let vptr = pbuf.ptr(j as usize);
                unsafe {
                    curprop.buf.push(*vptr);    
                }
                
            }
            props.push(curprop);
        }
    }
    Ok(props)
}

fn get_hw_infos(guid :* const GUID,flags :DWORD) -> Result<Vec<HwInfo>,Box<dyn Error>> {
    let mut retv :Vec<HwInfo> = Vec::new();
    let pinfo :HDEVINFO;
    unsafe {
        pinfo = SetupDiGetClassDevsW(guid,null_mut(),null_mut(),flags);    
    }

    let mut bret:BOOL;
    let mut cv :SP_DEVINFO_DATA ;
    let mut nindex : DWORD = 0;

    if pinfo == INVALID_HANDLE_VALUE{
        extargs_new_error!{WinSetupError,"can not create DeviceInfoList"}
    }

    loop {
        cv = SP_DEVINFO_DATA::default();
        cv.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
        unsafe{
            bret = SetupDiEnumDeviceInfo(pinfo,nindex,&mut cv);    
        }
        
        if bret == 0 {
            break;
        }
        let mut hwinfo = HwInfo {
            props : Vec::new(),
        };

        let ores = get_hw_props(pinfo,&mut cv);
        if ores.is_err() {
            if pinfo != INVALID_HANDLE_VALUE {
                unsafe{
                    SetupDiDestroyDeviceInfoList(pinfo);
                }                
            }   
            return Err(ores.err().unwrap());
        }
        hwinfo.props = ores.unwrap();
        retv.push(hwinfo);
        nindex += 1;
    }

    if pinfo != INVALID_HANDLE_VALUE {
        unsafe {
            SetupDiDestroyDeviceInfoList(pinfo);
        }        
    }   
    Ok(retv)        
}

fn output_hw_infos<W : std::io::Write>(fout :&mut W,hwinfos :&[HwInfo])  {
    let mut rets :String = "".to_string();
    let mut idx :usize = 0;
    let mut jdx :usize;
    let mut kidx :usize;
    let mut lasti :usize;

    while idx < hwinfos.len() {
        jdx = 0;
        while jdx < hwinfos[idx].props.len() {
            rets.push_str(&format!("nindex[{}].[{}] property[{}].[0x{:x}]", idx,jdx,hwinfos[idx].props[jdx].guid,hwinfos[idx].props[jdx].propidx));            
            kidx = 0;
            lasti = 0;
            while kidx < hwinfos[idx].props[jdx].buf.len() {
                if (kidx % 16) == 0 {
                    if kidx != lasti {
                        rets.push_str("    ");
                        while lasti != kidx {
                            let ch = hwinfos[idx].props[jdx].buf[lasti] ;
                            if ch >= 0x20 && ch <= 0x7e {
                                rets.push_str(&format!("{}",ch as char));
                            } else {
                                rets.push_str(".");
                            }
                            lasti += 1;
                        }
                    }
                    rets.push_str(&format!("\n0x{:08x}", kidx));
                }
                rets.push_str(&format!(" 0x{:02x}", hwinfos[idx].props[jdx].buf[kidx]));
                kidx += 1;
            }

            if kidx != lasti {
                while (kidx % 16) != 0 {
                    rets.push_str("     ");
                    kidx += 1;
                }
                rets.push_str("    ");
                while lasti < hwinfos[idx].props[jdx].buf.len() {
                    let ch = hwinfos[idx].props[jdx].buf[lasti] ;
                    if ch >= 0x20 && ch <= 0x7e {
                        rets.push_str(&format!("{}",ch as char));
                    } else {
                        rets.push_str(".");
                    }
                    lasti += 1;                    
                }
                rets.push_str("\n");
            }

            kidx = 0;
            let mut nv16 :Vec<u16>= Vec::new();

            while kidx < hwinfos[idx].props[jdx].buf.len() {
                while (kidx + 1) < hwinfos[idx].props[jdx].buf.len() {
                    let  mut curv :u16 = hwinfos[idx].props[jdx].buf[kidx] as u16 ;
                    curv += (hwinfos[idx].props[jdx].buf[kidx + 1] as u16) << 8;
                    if curv == 0 {
                        break;
                    }
                    nv16.push(curv);
                    kidx += 2;
                } 

                if nv16.len() > 0 {
                    rets.push_str(&format!("PROP    [{}]\n",wstr_to_str(&nv16)));
                }
                nv16 = Vec::new();
                kidx += 2;
            }
            jdx += 1;
        }
        if rets.len() > 0x1000 {
            let _ = fout.write_all(rets.as_bytes());
            rets = "".to_string();
        }
        idx += 1;
    }

    if rets.len() > 0 {
        let _ = fout.write_all(rets.as_bytes());
        //rets = "".to_string();
    }
    return
}

fn lshwinfo_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut ptrguid :* const GUID = null_mut();
    let mut guidget :GUID = GUID::default();
    let mut flags :DWORD = DIGCF_ALLCLASSES;


    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");

    if sarr.len() > 0 {
        guidget = parse_guid(&sarr[0])?;
        flags = DIGCF_PRESENT | DIGCF_DEVICEINTERFACE;
        ptrguid = &guidget;
        debug_buffer_trace!(ptrguid,std::mem::size_of::<GUID>(),"guid get");
    }
    debug_trace!("guid {:?}",guidget);
    let hwinfos = get_hw_infos(ptrguid,flags)?;

    if ns.get_string("output").len() > 0 {
        let fname = ns.get_string("output");
        let mut fout = std::fs::File::create(&fname)?;
        output_hw_infos(&mut fout,&hwinfos);
    } else {
        let mut of = std::io::stdout();
        output_hw_infos(&mut of,&hwinfos);
    }


    Ok(())
}

fn devpropset_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut offset :usize;
    let mut vidx :usize;
    let mut val :u8;
    let mut idx :usize;
    let maxsize :usize;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");

    maxsize =parse_u64(&sarr[0])? as usize;

    let mut cptr :*mut DEVPROPKEY ;
    let mut ptr :*mut u8;
    let mut curptr :*mut u8;
    let mut cvobj :AutoMem<DEVPROPKEY> = AutoMem::new(maxsize);
    idx = 0 ;
    while idx < maxsize {
        cptr = cvobj.ptr_mut(idx);
        unsafe {
            let _ = std::mem::take::<DEVPROPKEY>(&mut (*cptr));    
        }

        idx += 1;
    }


    idx = 1;
    while idx < sarr.len() {
        if (idx + 3) > sarr.len() {
            extargs_new_error!{WinSetupError,"need vidx offset value"}
        }
        vidx = parse_u64(&sarr[idx])? as usize;
        offset = parse_u64(&sarr[idx+1])? as usize;
        val = parse_u64(&sarr[idx+2])? as u8;
        vidx = vidx % cvobj.size();
        cptr = cvobj.ptr_mut(vidx);
        ptr = cptr as *mut u8;
        offset = offset % std::mem::size_of::<DEVPROPKEY>();
        unsafe {
            curptr = ptr.offset(offset as isize) as *mut u8;
            let _ = std::mem::replace::<u8>(&mut (*curptr), val);                
        }
        idx += 3;
    }        

    Ok(())
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: HANDLE,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut ::core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}

type FnNtQuerySystemInformation = extern "stdcall" fn(clstype :i32, pinfo :* mut c_void,bufsize :ULONG, pretsize :&mut ULONG) -> NTSTATUS;


struct ProcAddress { 
    hmod :HMODULE,
    p : * const CHAR,
}

extargs_error_class!{ProcAddressError}

impl Drop for ProcAddress {
    fn drop(&mut self) {
        if self.hmod != null_mut() {
            unsafe {FreeLibrary(self.hmod)};
        }
        self.hmod = null_mut();
        self.p = null();
    }
}

impl ProcAddress {
    #[allow(unused_assignments)]
    pub fn new(dllname :&str, procname :&str) -> Result<Self,Box<dyn Error>> {
        let mut hd :HMODULE;
        let mut vp :Vec<u8> = Vec::new();
        let mut idx :usize = 0;
        let bs :&[u8] = dllname.as_bytes();
        while idx < bs.len() {
            vp.push(bs[idx]);
            idx += 1;
        }
        vp.push(0);

        hd = unsafe {LoadLibraryA(vp.as_ptr() as *const CHAR)};
        if hd == null_mut() {
            let ret = unsafe{ GetLastError()};
            extargs_new_error!{ProcAddressError,"cannot load [{}] error [0x{:x}]",dllname,ret}
        }

        vp = Vec::new();
        idx = 0;
        let bs :&[u8]= procname.as_bytes();
        while idx < bs.len() {
            vp.push(bs[idx]);
            idx += 1;
        }
        vp.push(0);

        let p = unsafe {GetProcAddress(hd,vp.as_ptr() as *const CHAR) as *const CHAR};
        if p == null_mut() {
            let ret = unsafe{GetLastError()};
            unsafe{FreeLibrary(hd)};
            hd = null_mut();
            extargs_new_error!{ProcAddressError,"cannot find [{}] in [{}] error[0x{:x}]", procname,dllname,ret}
        }

        Ok(ProcAddress{
            hmod : hd,
            p : p
        })
    }

    pub fn get_procaddr(&self) -> *const CHAR {
        return self.p;
    }
}

unsafe impl Send for ProcAddress {}
unsafe impl Sync for ProcAddress {}


lazy_static !{
    #[allow(non_upper_case_globals)]
    static ref NT_QUERY_SYSTEM_INFORMATION_POINTER : ProcAddress = {
        ProcAddress::new("Ntdll.dll","NtQuerySystemInformation").unwrap()
    };
}

#[allow(unused_assignments)]
fn query_process() -> Result<(),Box<dyn Error>> {
    let mut inputbuf :* mut c_void = null_mut();
    let mut status :NTSTATUS ;
    let mut retsize :ULONG = 0;
    let inputsize :ULONG;
    let mut curproc :*const SYSTEM_PROCESS_INFORMATION;
    let queryfunc : FnNtQuerySystemInformation = unsafe {std::mem::transmute::<_,FnNtQuerySystemInformation>(NT_QUERY_SYSTEM_INFORMATION_POINTER.get_procaddr())};
    let mut cptr :*mut c_void;
    let mut c16 :Vec<u16>;

    status = queryfunc(5,inputbuf,0,&mut retsize);
    if retsize == 0 {
        extargs_new_error!{WinSetupError,"can not get size"}
    }

    inputsize = retsize;
    unsafe {
        inputbuf = malloc(inputsize as usize);    
    }    
    if inputbuf == null_mut() {
        extargs_new_error!{WinSetupError,"can not alloc size [{}]",retsize}
    }


    status = queryfunc(5,inputbuf,inputsize,&mut retsize);
    if status != 0 {
        unsafe {
            free(inputbuf);    
        }        
        inputbuf = null_mut();
        extargs_new_error!{WinSetupError,"can not query size [{}] error [0x{:x}]", inputsize,status}
    }

    curproc = inputbuf as *const SYSTEM_PROCESS_INFORMATION;
    loop {
        unsafe {
            if (*curproc).NextEntryOffset == 0 {
                break;
            }            
        }
        c16 = Vec::new();
        let mut idx :u16 = 0;
        let mut c8ptr :*const u16;
        unsafe {
            c8ptr = (*curproc).ImageName.Buffer as *const u16;    
            while idx < ((*curproc).ImageName.Length >> 1)  {
                if *c8ptr == 0 {
                    break;
                }
                c16.push(*c8ptr);
                c8ptr =  c8ptr.add(2);
                idx += 1;
            }            
        }
        debug_buffer_trace!(c16.as_ptr(),c16.len() * 2, "c16 buffer");
        let imgname = wstr_to_str(&c16);
        println!("imgname {}",imgname);
        cptr = curproc as *mut c_void;
        unsafe {
            cptr = cptr.add((*curproc).NextEntryOffset as usize);    
        }        
        curproc = cptr as *const SYSTEM_PROCESS_INFORMATION;
    }

    if inputbuf != null_mut() {
        unsafe {
            free(inputbuf);    
        }        
    }
    inputbuf = null_mut();
    println!("all over");

    Ok(())
}

fn queryproc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    init_log(ns.clone())?;
    let _ = query_process()?;
    Ok(())
}


#[allow(unused_variables)]
fn getprocaddr_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
        extargs_new_error!{WinSetupError,"need dllname procname"}
    }

    let procaddr :ProcAddress = ProcAddress::new(&sarr[0],&sarr[1])?;
    println!("{}:{} = {:p}", sarr[0],sarr[1],procaddr.get_procaddr());

    Ok(())
}


#[extargs_map_function(lshwinfo_handler,devpropset_handler,queryproc_handler,getprocaddr_handler)]
pub fn load_ecc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
        "lshwinfo<lshwinfo_handler>##[guids]... to list handle of guids##" : {
            "$" : "*"
        },
        "devpropset<devpropset_handler>##maxsize offset val ... to set offset value##" : {
            "$" : "+"
        },
        "queryproc<queryproc_handler>##to query process information##" : {
            "$" : 0
        },
        "getprocaddr<getprocaddr_handler>##dllname funcname to load function##" : {
            "$" : 2
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}