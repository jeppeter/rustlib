#! /usr/bin/env python

import extargsparse
import sys
import logging


def set_logging(args):
    loglvl= logging.ERROR
    if args.verbose >= 3:
        loglvl = logging.DEBUG
    elif args.verbose >= 2:
        loglvl = logging.INFO
    curlog = logging.getLogger(args.lognames)
    #sys.stderr.write('curlog [%s][%s]\n'%(args.logname,curlog))
    curlog.setLevel(loglvl)
    if len(curlog.handlers) > 0 :
        curlog.handlers = []
    formatter = logging.Formatter('%(asctime)s:%(filename)s:%(funcName)s:%(lineno)d<%(levelname)s>\t%(message)s')
    if not args.lognostderr:
        logstderr = logging.StreamHandler()
        logstderr.setLevel(loglvl)
        logstderr.setFormatter(formatter)
        curlog.addHandler(logstderr)

    for f in args.logfiles:
        flog = logging.FileHandler(f,mode='w',delay=False)
        flog.setLevel(loglvl)
        flog.setFormatter(formatter)
        curlog.addHandler(flog)
    for f in args.logappends:       
        if args.logrotate:
            flog = logging.handlers.RotatingFileHandler(f,mode='a',maxBytes=args.logmaxbytes,backupCount=args.logbackupcnt,delay=0)
        else:
            sys.stdout.write('appends [%s] file\n'%(f))
            flog = logging.FileHandler(f,mode='a',delay=0)
        flog.setLevel(loglvl)
        flog.setFormatter(formatter)
        curlog.addHandler(flog)
    return


def load_log_commandline(parser):
    logcommand = '''
    {
        "verbose|v" : "+",
        "logname" : "root",
        "logfiles" : [],
        "logappends" : [],
        "logrotate" : true,
        "logmaxbytes" : 10000000,
        "logbackupcnt" : 2,
        "lognostderr" : false
    }
    '''
    parser.load_command_line_string(logcommand)
    return parser

def parse_int(v):
    c = v
    base = 10
    if c.startswith('0x') or c.startswith('0X') :
        base = 16
        c = c[2:]
    elif c.startswith('x') or c.startswith('X'):
        base = 16
        c = c[1:]
    return int(c,base)

def write_file(s,outfile=None):
    fout = sys.stdout
    if outfile is not None:
        fout = open(outfile, 'w+b')
    outs = s
    if 'b' in fout.mode:
        outs = s.encode('utf-8')
    fout.write(outs)
    if fout != sys.stdout:
        fout.close()
    fout = None
    return 

def write_file_bytes(sarr,outfile=None):
    fout = sys.stdout
    if outfile is not None:
        fout = open(outfile, 'wb')
    if 'b' not in fout.mode:
        fout.buffer.write(sarr)
    else:        
        fout.write(sarr)
    if fout != sys.stdout:
        fout.close()
    fout = None
    return 

def format_tab_line(tab,s):
    rets = ''
    for i in range(tab):
        rets += '    '
    rets += s
    rets += '\n'
    return rets



def genrustint_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        outs += format_tab_line(2,'idx = 3;')
        outs += format_tab_line(2,'while idx < sarr.len() {')
        outs += format_tab_line(3,'params.push(parse_u64(&sarr[idx])? as i32);')
        outs += format_tab_line(3,'idx += 1;')
        outs += format_tab_line(2,'}')

        outs += format_tab_line(2,'while (params.len() as u64) < numparam {')
        outs += format_tab_line(3,'params.push(0);')
        outs += format_tab_line(2,'}')
        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'i32'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(%s) -> i32 > = lib.get(funcname.as_bytes())?;'%(curs))
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d]'%(j)
            j += 1
        outs += format_tab_line(3,'retval = func(%s);'%(curs))
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return

def genrustptr_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        outs += format_tab_line(2,'idx = 3;')
        outs += format_tab_line(2,'while idx < sarr.len() {')
        outs += format_tab_line(3,'params.push(parse_u64(&sarr[idx])? as i32);')
        outs += format_tab_line(3,'idx += 1;')
        outs += format_tab_line(2,'}')

        outs += format_tab_line(2,'while (params.len() as u64) < numparam {')
        outs += format_tab_line(3,'params.push(0);')
        outs += format_tab_line(2,'}')
        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'i32'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(%s) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;'%(curs))
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d]'%(j)
            j += 1
        outs += format_tab_line(3,'retval = func(%s);'%(curs))
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return


def genruststr_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        outs += format_tab_line(2,'idx = 3;')
        outs += format_tab_line(2,'while idx < sarr.len() {')
        outs += format_tab_line(3,'params.push(format!("{}\\0",sarr[idx]));')
        outs += format_tab_line(3,'idx += 1;')
        outs += format_tab_line(2,'}')

        outs += format_tab_line(2,'while (params.len() as u64) < numparam {')
        outs += format_tab_line(3,'params.push(format!("\\0"));')
        outs += format_tab_line(2,'}')
        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += '*const std::ffi::c_char'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(%s) -> *const std::ffi::c_void > = lib.get(funcname.as_bytes())?;'%(curs))
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d].as_ptr() as *const std::ffi::c_char'%(j)
            j += 1
        outs += format_tab_line(3,'retval = func(%s);'%(curs))
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return

def genrustfncall_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        outs += format_tab_line(0,'')
        outs += format_tab_line(0,'// callback function with %d params'%(i))
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'a%d :*const std::ffi::c_char'%(j)
            j += 1
        outs += format_tab_line(0,'unsafe extern "C" fn callback_%d(%s) -> std::ffi::c_int {'%(i,curs))
        if i > 0:
            j = 0
            while j < i:
                outs += format_tab_line(1,'let a%ds : String;'%(j))
                j += 1
            if i > 1:
                outs += format_tab_line(1,'let mut ores:Result<&str,std::str::Utf8Error>;')
            else:
                outs += format_tab_line(1,'let ores:Result<&str,std::str::Utf8Error>;')
            j = 0
            while j < i:
                outs += format_tab_line(1,' ')
                outs += format_tab_line(1,'ores = unsafe { std::ffi::CStr::from_ptr(a%d).to_str()};'%(j))
                outs += format_tab_line(1,'if ores.is_err() {')
                outs += format_tab_line(2,'println!("RUST: error on %d");'%(j))
                outs += format_tab_line(2,'return -1;')
                outs += format_tab_line(1,'}')
                outs += format_tab_line(1,'a%ds = ores.unwrap().to_string();'%(j))
                outs += format_tab_line(1,'println!("RUST:a%d={}",a%ds);'%(j,j))
                j += 1
        outs += format_tab_line(1,'return %d;'%(i))
        outs += format_tab_line(0,'}')

        i += 1

    write_file(outs,args.output)
    sys.exit(0)
    return

def genrustcallback_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += '*const std::ffi::c_char'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(%s) -> std::ffi::c_int,%s) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;'%(curs,curs))
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d].as_ptr() as *const std::ffi::c_char'%(j)
            j += 1
        if i > 0:
            outs += format_tab_line(3,'retval = func(callback_%d,%s);'%(i,curs))
        else:
            outs += format_tab_line(3,'retval = func(callback_%d);'%(i))
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return


def genruststkcallback_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += '*const std::ffi::c_char'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        if i > 0:
            outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,%s) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;'%(curs))
        else:
            outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;')
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d].as_ptr() as *const std::ffi::c_char'%(j)
            j += 1
        if i > 0:
            outs += format_tab_line(3,'retval = func(stk_call_back,%s);'%(curs))
        else:
            outs += format_tab_line(3,'retval = func(stk_call_back);')
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return

def genruststkcallbackargs_handler(args,parser):
    set_logging(args)
    num = 10
    prefix = 'params'
    if len(args.subnargs) > 0:
        num = parse_int(args.subnargs[0])
    if len(args.subnargs) > 1:
        prefix = args.subnargs[1]

    i = 0
    outs = ''
    while i < num:
        if i == 0:
            outs += format_tab_line(1,'if numparam == %d {'%(i))
        else:
            outs += format_tab_line(1,'} else if numparam == %d {'%(i))

        curs = ''        
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += '*const std::ffi::c_char'
            j += 1
        outs += format_tab_line(2,'unsafe {')
        if i > 0:
            outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void,%s) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;'%(curs))
        else:
            outs += format_tab_line(3,'let func :libloading::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(*const std::ffi::c_void,std::ffi::c_int,*const *const std::ffi::c_char) -> std::ffi::c_int,*const std::ffi::c_void) -> std::ffi::c_int > = lib.get(funcname.as_bytes())?;')
        curs = ''
        j = 0
        while j < i:
            if j > 0:
                curs += ','
            curs += 'params[%d].as_ptr() as *const std::ffi::c_char'%(j)
            j += 1
        if i > 0:
            outs += format_tab_line(3,'retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void,%s);'%(curs))
        else:
            outs += format_tab_line(3,'retval = func(stk_call_back_with_arg,&ptrcc as *const CCValue as *const std::ffi::c_void);')
        outs += format_tab_line(2,'}')
        i += 1
    outs += format_tab_line(1,'} else {')
    outs += format_tab_line(2,'extargs_new_error!{DlError,"not supported {}",numparam}')
    outs += format_tab_line(1,'}')

    write_file(outs,args.output)
    sys.exit(0)
    return


def main():
    commandline='''
    {
        "input|i" : null,
        "output|o" : null,
        "genrustint<genrustint_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genrustptr<genrustptr_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genruststr<genruststr_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genrustfncall<genrustfncall_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genrustcallback<genrustcallback_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genruststkcallback<genruststkcallback_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        },
        "genruststkcallbackargs<genruststkcallbackargs_handler>##[num] [prefix] to generate function with default prefix print num default 10##" : {
            "$" : "*"
        }
    }
    '''
    parser = extargsparse.ExtArgsParse()
    load_log_commandline(parser)
    parser.load_command_line_string(commandline)
    parser.parse_command_line(None,parser)
    raise Exception('can not reach here')
    return

if __name__ == '__main__':
    main()