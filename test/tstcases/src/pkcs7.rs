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
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};


use super::fileop::{read_file,read_file_bytes,write_file_bytes};
use super::pemlib::{pem_to_der,der_to_pem};
use super::cryptlib::{aes256_cbc_decrypt};
use super::asn1def::*;
use asn1obj::base::{Asn1OctData,Asn1Any,Asn1Object};
use asn1obj::complex::{Asn1Seq};
use asn1obj::asn1impl::{Asn1Op};
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use sha2::{Sha256,Digest};
use hmac::{Hmac,Mac};
use hex::FromHex;
use rsa::{RsaPublicKey,RsaPrivateKey,PublicKey};
use rsa::BigUint as rsaBigUint;
use rsa::hash::{Hash};
use rsa::padding::{PaddingScheme};

asn1obj_error_class!{Pkcs7Error}



fn pkcs7dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {    
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pkcs7::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("xname",0,&mut f)?;
    }

    Ok(())
}

fn x509namedec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> { 
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1X509Name::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("xname",0,&mut f)?;
    }

    Ok(())
}


fn objenc_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut objd :Asn1Object = Asn1Object::init_asn1();

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let _ = objd.set_value(f)?;
        let vcode = objd.encode_asn1()?;
        debug_buffer_trace!(vcode.as_ptr(), vcode.len(), "encode {} object", f);
    }

    Ok(())
}

fn objdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let mut objd :Asn1Object = Asn1Object::init_asn1();

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let v8 :Vec<u8> = Vec::from_hex(f).unwrap();
        let _ = objd.decode_asn1(&v8)?;
        println!("{} => {}", f, objd.get_value());
    }

    Ok(())
}

fn pemtoder_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {    
    let inf :String;
    let outf :String;

    init_log(ns.clone())?;
    inf = ns.get_string("input");
    outf = ns.get_string("output");

    let s = read_file(&inf)?;

    let (bb,notice) = pem_to_der(&s)?;
    debug_trace!("notice {}",notice);
    let _ = write_file_bytes(&outf,&bb)?;
    Ok(())
}

fn dertopem_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {    
    let inf :String;
    let outf :String;
    let sarr :Vec<String>;
    let mut notice :String = "NOTE".to_string();

    init_log(ns.clone())?;

    sarr = ns.get_array("subnargs");
    if sarr.len() > 0 {
        notice = format!("{}",sarr[0]);
    }

    inf = ns.get_string("input");
    outf = ns.get_string("output");

    let inb = read_file_bytes(&inf)?;

    let outs = der_to_pem(&inb,&notice)?;
    let _ = write_file_bytes(&outf,outs.as_bytes())?;
    Ok(())
}



fn privinfodec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> { 
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pkcs8PrivKeyInfo::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("privinfo",0,&mut f)?;     
    }

    Ok(())
}


fn encryptprivdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1X509Sig::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("x509sig",0,&mut f)?;      
    }

    Ok(())
}


fn pbe2dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> { 
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pbe2ParamElem::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("pbe2param",0,&mut f)?;        
    }

    Ok(())
}



fn pbkdf2dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pbkdf2ParamElem::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("pbkdf2param",0,&mut f)?;      
    }

    Ok(())
}

type HmacSha256 = Hmac<Sha256>;

fn get_hmac_sha256_key(passv8 :&[u8], saltv8 :&[u8], itertimes : usize) -> Vec<u8> {
    let omac = HmacSha256::new_from_slice(&passv8).unwrap();
    let mut nmac ;
    let mut tkeylen : usize = 32;
    let cplen :usize = 32;
    let mut i :usize = 1;
    let mut p :Vec<u8> = Vec::new();
    let mut plen :usize = 0;

    while tkeylen > 0 {
        let mut itmp :Vec<u8> = Vec::new();
        let mut curv :u8;
        nmac = omac.clone();
        curv = ((i >> 24) & 0xff) as u8;
        itmp.push(curv);
        curv = ((i >> 16) & 0xff) as u8;
        itmp.push(curv);
        curv = ((i >> 8) & 0xff) as u8;
        itmp.push(curv);
        curv = ((i >> 0) & 0xff) as u8;
        itmp.push(curv);
        nmac.update(&saltv8);
        nmac.update(&itmp);
        let mut resdigtmp = nmac.finalize();
        let mut digtmp = resdigtmp.into_bytes();
        for i in 0..digtmp.len() {
            if (p.len()-plen) <= i {
                p.push(digtmp[i]);
            } else {
                p[i+plen] = digtmp[i];
            }
        }


        for _ in 1..itertimes {
            nmac = omac.clone();
            nmac.update(&digtmp);
            resdigtmp = nmac.finalize();
            digtmp = resdigtmp.into_bytes();
            for k in 0..cplen {
                p[k+plen] ^= digtmp[k];
            }
        }

        tkeylen -= cplen;
        i += 1;
        plen += cplen;
    }
    return p;   
}

fn hmacsha256_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 2 {
        asn1obj_new_error!{Pkcs7Error,"need password salt "}
    }
    let passv8 :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
    let saltv8 :Vec<u8> = Vec::from_hex(&sarr[1]).unwrap();
    debug_trace!("passv8 {:?} saltv8 {:?}", passv8,saltv8);
    let p = get_hmac_sha256_key(&passv8,&saltv8,2048);
    debug_buffer_trace!(p.as_ptr(),p.len(),"final p");
    Ok(())
}


fn netpkeydec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut xname = Asn1NetscapePkey::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("netpkey",0,&mut f)?;      
    }

    Ok(())
}


fn get_private_key(x509bytes :&[u8],passin :&[u8]) -> Result<Asn1RsaPrivateKey,Box<dyn Error>> {
    let mut x509sig = Asn1X509Sig::init_asn1();
    let _ = x509sig.decode_asn1(x509bytes)?;
    let types = x509sig.elem.val[0].algor.elem.val[0].algorithm.get_value();
    if types == OID_PBES2 {
        debug_trace!("debug {}", OID_PBES2);
        let params :&Asn1Any = x509sig.elem.val[0].algor.elem.val[0].parameters.val.as_ref().unwrap();
        let decdata :Vec<u8> = params.content.clone();
        let mut pbe2 : Asn1Pbe2ParamElem = Asn1Pbe2ParamElem::init_asn1();
        let _ = pbe2.decode_asn1(&decdata)?;
        let pbe2types = pbe2.keyfunc.elem.val[0].algorithm.get_value();
        if pbe2types == OID_PBKDF2 {
            debug_trace!("debug {}", OID_PBKDF2);
            let params :&Asn1Any = pbe2.keyfunc.elem.val[0].parameters.val.as_ref().unwrap();
            let decdata :Vec<u8> = params.content.clone();
            let mut pbkdf2 :Asn1Pbkdf2ParamElem = Asn1Pbkdf2ParamElem::init_asn1();
            let _ = pbkdf2.decode_asn1(&decdata)?;
            let aeskey :Vec<u8> = get_hmac_sha256_key(passin,&pbkdf2.salt.content,pbkdf2.iter.val as usize);
            if pbe2.encryption.elem.val[0].algorithm.get_value() == OID_AES_256_CBC {
                let params :Asn1Any = pbe2.encryption.elem.val[0].parameters.val.as_ref().unwrap().clone();
                let ivkey :Vec<u8> = params.content.clone();
                let encdata :Vec<u8> = x509sig.elem.val[0].digest.data.clone();
                let decdata :Vec<u8> = aes256_cbc_decrypt(&encdata,&aeskey,&ivkey)?;
                let mut netpkey :Asn1NetscapePkey = Asn1NetscapePkey::init_asn1();
                let _ = netpkey.decode_asn1(&decdata)?;
                if netpkey.elem.val[0].algor.elem.val[0].algorithm.get_value() == OID_RSA_ENCRYPTION {
                    let decdata :Vec<u8> = netpkey.elem.val[0].privdata.data.clone();
                    let mut privkey :Asn1RsaPrivateKey = Asn1RsaPrivateKey::init_asn1();
                    let _ = privkey.decode_asn1(&decdata)?;
                    return Ok(privkey);
                }
            }
        }
    }

    asn1obj_new_error!{Pkcs7Error,"not private key"}
}

fn get_private_key_file(pemfile :&str,passin :&[u8]) -> Result<Asn1RsaPrivateKey,Box<dyn Error>> {
    let pemdata = read_file(pemfile)?;
    let (derdata,_) = pem_to_der(&pemdata)?;
    return get_private_key(&derdata,passin);
}

fn get_rsa_private_key(pemfile :&str, passin :&[u8]) -> Result<RsaPrivateKey, Box<dyn Error>> {
    let privkey = get_private_key_file(pemfile,passin)?;
    let n = rsaBigUint::from_bytes_be(&privkey.elem.val[0].modulus.val.to_bytes_be());
    let d = rsaBigUint::from_bytes_be(&privkey.elem.val[0].pubexp.val.to_bytes_be());
    let e = rsaBigUint::from_bytes_be(&privkey.elem.val[0].privexp.val.to_bytes_be());
    let mut primes :Vec<rsaBigUint> = Vec::new();
    primes.push(rsaBigUint::from_bytes_be(&privkey.elem.val[0].prime1.val.to_bytes_be()));
    primes.push(rsaBigUint::from_bytes_be(&privkey.elem.val[0].prime2.val.to_bytes_be()));
    let po = RsaPrivateKey::from_components(n,d,e,primes);
    Ok(po)
}

fn rsaprivdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let passin :String = ns.get_string("passin");
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let privkey = get_private_key(&code,passin.as_bytes())?;
        let mut f = std::io::stderr();
        privkey.print_asn1("privkey",0,&mut f)?;
    }

    Ok(())
}


fn get_sha256_data(ind :&[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&ind);
    let res = hasher.finalize();
    return res.to_vec();    
}

fn sha256_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let data = read_file_bytes(f)?;
        let res = get_sha256_data(&data);
        debug_buffer_trace!(res.as_ptr(),res.len(),"file [{}] sha256", f);
    }
    Ok(())
}


fn rsaverify_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {   
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");

    if sarr.len() < 2 {
        asn1obj_new_error!{Pkcs7Error,"should indata signdata file"}
    }
    let passin = ns.get_string("passin");
    let keyfile = ns.get_string("rsapriv");
    if keyfile.len() == 0 {
        asn1obj_new_error!{Pkcs7Error,"need rsa private key file"}
    }
    let privkey = get_rsa_private_key(&keyfile,passin.as_bytes())?;
    let pubkey = privkey.to_public_key();

    let ind = read_file_bytes(&sarr[0])?;
    let hashd = read_file_bytes(&sarr[1])?;
    let digest = get_sha256_data(&ind);
    let ro = pubkey.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,&hashd);
    match ro {
        Ok(_v) => {
            println!("verify [{}] with [{}] ok", sarr[0],sarr[1]);
        },
        Err(e) => {
            asn1obj_new_error!{Pkcs7Error,"failed verify [{}] with [{}] {:?}",sarr[0],sarr[1],e}
        }
    }
    Ok(())
}

fn x509dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> { 
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let fname = format!("{}",f);
        let mut xname = Asn1X509::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("x509",0,&mut f)?;
        let v8 = xname.elem.val[0].certinfo.encode_asn1()?;
        debug_buffer_trace!(v8.as_ptr(),v8.len(),"encode certinfo");
        let certinfoelem = xname.elem.val[0].certinfo.elem.val[0].clone();
        if certinfoelem.signature.elem.val[0].algorithm.get_value() == OID_SHA256_WITH_RSA_ENCRYPTION {
            /*now get the public key*/
            let pubn = certinfoelem.key.elem.val[0].rsa.val.val[0].n.val.to_bytes_be();
            let pube = certinfoelem.key.elem.val[0].rsa.val.val[0].e.val.to_bytes_be();
            let rsapubk = RsaPublicKey::new(rsaBigUint::from_bytes_be(&pubn),rsaBigUint::from_bytes_be(&pube))?;
            let digest = get_sha256_data(&v8);
            let hmactype = xname.elem.val[0].sig_alg.elem.val[0].algorithm.get_value();

            debug_trace!("sig_alg value [{}]", hmactype);
            if hmactype == OID_SHA256_WITH_RSA_ENCRYPTION {
                let hashd = xname.elem.val[0].signature.data.clone();
                let ro = rsapubk.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,&hashd);
                match ro {
                    Ok(_) => {
                        println!("{} is ok", fname);
                    },
                    Err(_e) => {                    
                        eprintln!("{} not ok {:?}",fname,_e);
                    }
                }
            }
        }
    }

    Ok(())
}


fn csrdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let fname = format!("{}",f);
        let code = read_file_bytes(f)?;
        let mut xname = Asn1X509Req::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("x509req",0,&mut f)?;      
        let v8 = xname.elem.val[0].req_info.encode_asn1()?;
        debug_buffer_trace!(v8.as_ptr(),v8.len(),"encode certinfo");
        let reqinfoelem = xname.elem.val[0].req_info.elem.val[0].clone();
        if reqinfoelem.pubkey.elem.val[0].valid.val.val.get_value() == OID_RSA_ENCRYPTION {
            /*now get the public key*/
            let pubkey : Asn1RsaPubkey = reqinfoelem.pubkey.elem.val[0].rsa.val.val[0].clone();
            let pubn = pubkey.n.val.to_bytes_be();
            let pube = pubkey.e.val.to_bytes_be();
            let rsapubk = RsaPublicKey::new(rsaBigUint::from_bytes_be(&pubn),rsaBigUint::from_bytes_be(&pube))?;
            let digest = get_sha256_data(&v8);
            let hmactype = xname.elem.val[0].sig_alg.elem.val[0].algorithm.get_value();

            debug_trace!("sig_alg value [{}]", hmactype);
            if hmactype == OID_SHA256_WITH_RSA_ENCRYPTION {
                let hashd = xname.elem.val[0].signature.data.clone();
                let ro = rsapubk.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,&hashd);
                match ro {
                    Ok(_) => {
                        println!("{} is ok", fname);
                    },
                    Err(_e) => {                    
                        eprintln!("{} not ok {:?}",fname,_e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn pkcs12dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        //let fname = format!("{}",f);
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pkcs12::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("pkcs12",0,&mut f)?;
    }

    Ok(())
}

fn authsafesdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut octdata :Asn1OctData = Asn1OctData::init_asn1();
        let size = octdata.decode_asn1(&code)?;
        debug_trace!("size [0x{:x}:{}]", size,size);
        let v8 = octdata.data.clone();
        let mut f = std::io::stderr();
        let mut safes :Asn1AuthSafes = Asn1AuthSafes::init_asn1();
        let rlen = safes.decode_asn1(&v8)?;
        debug_trace!("rlen [{}:0x{:x}]", rlen,rlen);
        let _ = safes.print_asn1("safes", 0, &mut f)?;
    }
    Ok(())
}

fn pkcs12safebagdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut octdata :Asn1Seq<Asn1Pkcs12SafeBag> = Asn1Seq::init_asn1();
        let _ = octdata.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        let _ = octdata.print_asn1("safebag", 0, &mut f)?;
    }
    Ok(())
}

#[extargs_map_function(pkcs7dec_handler,x509namedec_handler,objenc_handler,pemtoder_handler,dertopem_handler,encryptprivdec_handler,privinfodec_handler,pbe2dec_handler,pbkdf2dec_handler,hmacsha256_handler,netpkeydec_handler,rsaprivdec_handler,x509dec_handler,sha256_handler,rsaverify_handler,csrdec_handler,pkcs12dec_handler,objdec_handler,authsafesdec_handler,pkcs12safebagdec_handler)]
pub fn load_pkcs7_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
        "passin" : null,
        "rsapriv"  : null,
        "pkcs7dec<pkcs7dec_handler>##derfile ... to decode file##" : {
            "$" : "+"
        },
        "x509namedec<x509namedec_handler>##derfile ... to decode file##" : {
            "$" : "+"
        },
        "objenc<objenc_handler>##objid ... to encode object##" : {
            "$" : "+"
        },
        "pemtoder<pemtoder_handler>##to tranform input to output from pem to der##" : {
            "$" : 0
        },
        "dertopem<dertopem_handler>##[NOTICE] to tranform input to output from der to pem##" : {
            "$" : "?"
        },
        "encryptprivdec<encryptprivdec_handler>##derfile ... to decode file##" : {
            "$" : "+"
        },
        "privinfodec<privinfodec_handler>##derfile ... to decode file##" : {
            "$" : "+"
        },
        "pbe2dec<pbe2dec_handler>##derfile ... to decode PBE2PARAM##" : {
            "$" : "+"
        },
        "pbkdf2dec<pbkdf2dec_handler>##derfile ... to decode PBKDF2PARAM##" : {
            "$" : "+"
        },
        "hmacsha256<hmacsha256_handler>##password salt to encode##" : {
            "$" : 2
        },
        "netpkeydec<netpkeydec_handler>##derfile ... to decode NETSCAPEPKEY##" : {
            "$" : "+"
        },
        "rsaprivdec<rsaprivdec_handler>##derfile ... to decode RSAPRIVATEKEY##" : {
            "$" : "+"
        },
        "x509dec<x509dec_handler>##derfile ... to decode X509##" : {
            "$" : "+"
        },
        "sha256<sha256_handler>##infile ... to sha256 file##" : {
            "$" : "+"
        },
        "rsaverify<rsaverify_handler>##infile rsasignval to verify file to get the file##" : {
            "$" : 2
        },
        "csrdec<csrdec_handler>##derfile ... to decode X509_REQ##" : {
            "$" : "+"
        },
        "pkcs12dec<pkcs12dec_handler>##derfile ... to decode PKCS12##" : {
            "$" : "+"
        },
        "objdec<objdec_handler>##hexstr ... to decode in hexstr##" : {
            "$" : "+"
        },
        "authsafesdec<authsafesdec_handler>##derfile .... to dec ASN1_OCT_DATA [PKCS7_ENCRYPT]##" : {
            "$" : "+"
        },
        "pkcs12safebagdec<pkcs12safebagdec_handler>##derfile .... to dec PKCS12_SAFEBAG##" : {
            "$" : "+"
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}