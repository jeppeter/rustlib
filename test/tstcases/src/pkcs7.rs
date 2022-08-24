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
use std::io::{Write};

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
use super::strop::{decode_base64};

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

fn get_algor_pbkdf2_private_data(x509algorbytes :&[u8],encdata :&[u8],passin :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut algor :Asn1X509Algor = Asn1X509Algor::init_asn1();
    let _ = algor.decode_asn1(x509algorbytes)?;
    let types = algor.elem.val[0].algorithm.get_value();
    if types == OID_PBES2 {
        let params :&Asn1Any = algor.elem.val[0].parameters.val.as_ref().unwrap();
        let decdata :Vec<u8> = params.content.clone();
        let mut pbe2 : Asn1Pbe2ParamElem = Asn1Pbe2ParamElem::init_asn1();
        let _ = pbe2.decode_asn1(&decdata)?;
        let pbe2types = pbe2.keyfunc.elem.val[0].algorithm.get_value();
        if pbe2types == OID_PBKDF2 {
            //debug_trace!("debug {}", OID_PBKDF2);
            let params :&Asn1Any = pbe2.keyfunc.elem.val[0].parameters.val.as_ref().unwrap();
            let decdata :Vec<u8> = params.content.clone();
            let mut pbkdf2 :Asn1Pbkdf2ParamElem = Asn1Pbkdf2ParamElem::init_asn1();
            let _ = pbkdf2.decode_asn1(&decdata)?;
            let aeskey :Vec<u8> = get_hmac_sha256_key(passin,&pbkdf2.salt.content,pbkdf2.iter.val as usize);
            let types = pbe2.encryption.elem.val[0].algorithm.get_value();
            if types  == OID_AES_256_CBC {
                let params :Asn1Any = pbe2.encryption.elem.val[0].parameters.val.as_ref().unwrap().clone();
                let ivkey :Vec<u8> = params.content.clone();
                let decdata :Vec<u8> = aes256_cbc_decrypt(encdata,&aeskey,&ivkey)?;
                return Ok(decdata);
            }
            asn1obj_new_error!{Pkcs7Error,"not support OID_PBKDF2 types [{}]", types}
        }
        asn1obj_new_error!{Pkcs7Error,"not support OID_PBES2 types [{}]",pbe2types}
    }
    asn1obj_new_error!{Pkcs7Error,"can not support types [{}]", types}
}

fn get_private_key(x509sigbytes :&[u8],passin :&[u8]) -> Result<Asn1RsaPrivateKey,Box<dyn Error>> {
    let mut x509sig = Asn1X509Sig::init_asn1();
    let _ = x509sig.decode_asn1(x509sigbytes)?;
    let algordata = x509sig.elem.val[0].algor.encode_asn1()?;
    let encdata = x509sig.elem.val[0].digest.data.clone();
    let decdata = get_algor_pbkdf2_private_data(&algordata,&encdata,passin)?;
    let mut netpkey :Asn1NetscapePkey = Asn1NetscapePkey::init_asn1();
    let _ = netpkey.decode_asn1(&decdata)?;
    let types = netpkey.elem.val[0].algor.elem.val[0].algorithm.get_value();
    if types == OID_RSA_ENCRYPTION {
        let decdata :Vec<u8> = netpkey.elem.val[0].privdata.data.clone();
        let mut privkey :Asn1RsaPrivateKey = Asn1RsaPrivateKey::init_asn1();
        let _ = privkey.decode_asn1(&decdata)?;
        return Ok(privkey);
    }
    asn1obj_new_error!{Pkcs7Error,"not support [{}]",types}
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

fn get_pubk_value(code :&[u8]) -> Result<Asn1RsaPubkey,Box<dyn Error>> {
    let mut pubkform = Asn1RsaPubkeyForm::init_asn1();
    let _ = pubkform.decode_asn1(code)?;
    let mut pubk = Asn1RsaPubkey::init_asn1();
    let types = pubkform.elem.val[0].algor.elem.val[0].algorithm.get_value();
    if types == OID_RSA_ENCRYPTION {
        let _ = pubk.decode_asn1(&(pubkform.elem.val[0].data.data))?;
    } else {
        asn1obj_new_error!{Pkcs7Error,"not valid code for pubk"}
    }
    Ok(pubk)
}

fn get_pubk_from_file(f :&str) -> Result<Asn1RsaPubkey,Box<dyn Error>> {
    let code = read_file_bytes(f)?;
    let ro = get_pubk_value(&code);
    if ro.is_ok() {
        let pubk = ro.unwrap();
        return Ok(pubk);
    }

    let cs = read_file(f)?;
    let (code,_) = pem_to_der(&cs)?;
    return get_pubk_value(&code);
}

fn rsapubdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let pubk = get_pubk_value(&code)?;
        let mut f = std::io::stderr();
        let _ = pubk.print_asn1("pubk",0,&mut f);
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
    let capub :String;

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    capub = ns.get_string("capub");
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
            let pubn ;
            let pube ;
            if capub.len() == 0 {
                debug_trace!("self signed key");
                pubn = certinfoelem.key.elem.val[0].rsa.val.elem.val[0].n.val.to_bytes_be();
                pube = certinfoelem.key.elem.val[0].rsa.val.elem.val[0].e.val.to_bytes_be();
            } else {
                let pubk = get_pubk_from_file(&capub)?;
                debug_trace!("pubkey from [{}]",capub);
                pubn = pubk.elem.val[0].n.val.to_bytes_be();
                pube = pubk.elem.val[0].e.val.to_bytes_be();
            }
            let rsapubk = RsaPublicKey::new(rsaBigUint::from_bytes_be(&pubn),rsaBigUint::from_bytes_be(&pube))?;
            let digest = get_sha256_data(&v8);
            let hmactype = xname.elem.val[0].sig_alg.elem.val[0].algorithm.get_value();

            debug_trace!("sig_alg value [{}]", hmactype);
            if hmactype == OID_SHA256_WITH_RSA_ENCRYPTION {
                //let hashd = xname.elem.val[0].signature.data.clone();
                let ro = rsapubk.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,&xname.elem.val[0].signature.data);
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
            let pubkey : Asn1RsaPubkeyElem = reqinfoelem.pubkey.elem.val[0].rsa.val.elem.val[0].clone();
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
                        let c = format!("{} is ok\n", fname);
                        let _ = f.write(c.as_bytes())?;
                    },
                    Err(_e) => {                    
                        let c = format!("{} not ok {:?}",fname,_e);
                        let _ = f.write(c.as_bytes())?;
                    }
                }
            }
        }
    }

    Ok(())
}

const PKCS12_MAC_ID :u8 = 3;
const SHA256_BLOCK_SIZE :usize = 64;
const SHA256_DIGEST_SIZE :usize = 32;

#[allow(non_snake_case)]
fn get_pkcs12kdf_sha256(passin :&[u8],salt :&[u8], idval :u8, iterval :usize,totaln :usize) -> Vec<u8> {
    let mut Darr :Vec<u8> = Vec::new();
    let mut Aiarr :Vec<u8> = Vec::new();
    let mut Barr :Vec<u8> = Vec::new();
    let mut Iarr :Vec<u8> = Vec::new();
    let mut retv :Vec<u8> = Vec::new();
    let v :usize = SHA256_BLOCK_SIZE;
    let u :usize = SHA256_DIGEST_SIZE;
    let slen :usize = v * ((salt.len() + v - 1) / v);
    let  plen :usize ;
    if passin.len() != 0 {
        plen = v * ((passin.len() + v - 1) / v);
    } else {
        plen = 0;
    }
    let ilen :usize = slen + plen;

    for _ in 0..v {
        Darr.push(idval);
    }

    for i in 0..slen {
        Iarr.push(salt[ (i % salt.len()) ]);
    }

    for i in 0..plen {
        Iarr.push(passin[(i % passin.len())]);
    }

    for  _ in 0..(v + 1) {
        Barr.push(0);
    }

    for _ in 0..(u) {
        Aiarr.push(0);
    }

    loop {
        let mut hasher = Sha256::new();
        hasher.update(&Darr[0..v]);
        hasher.update(&Iarr[0..ilen]);
        let res = hasher.finalize();
        let rdata = res.to_vec();
        for i in 0..rdata.len() {
            Aiarr[i] = rdata[i];
        }

        debug_buffer_trace!(Darr.as_ptr(),u,"Darr Data");
        debug_buffer_trace!(Iarr.as_ptr(),Iarr.len(), "Iarr Data");

        debug_buffer_trace!(Aiarr.as_ptr(),Aiarr.len(),"Aiarr Data");
        for _ in 1..iterval {
            let mut hasher = Sha256::new();
            hasher.update(&Aiarr[0..u]);
            let res = hasher.finalize();
            let rdata = res.to_vec();
            for i in 0..rdata.len() {
                Aiarr[i] = rdata[i];
            }
        }
        debug_buffer_trace!(Aiarr.as_ptr(),Aiarr.len(),"Aiarr Data");

        for i in 0..u {
            if retv.len() >= totaln {
                break;
            }
            retv.push(Aiarr[i]);
        }

        if retv.len() >= totaln {
            break;
        }

        for j in 0..v {
            Barr[j] = Aiarr[(j % u)];
        }
        let mut jdx :usize = 0;
        while jdx < ilen {
            let mut k :usize = v;
            let mut c :u16 = 1;
            while k > 0 {
                k -= 1;
                c += Iarr[(k+jdx)] as u16 + Barr[k] as u16;
                Iarr[(k + jdx)] = c as u8;
                c >>= 8;
            }
            jdx += v;
        }
    }

    return retv;
}

fn calc_hmac_sha256(initkey :&[u8],data :&[u8]) -> Vec<u8> {
    let mut hmac = HmacSha256::new_from_slice(initkey).unwrap();
    hmac.update(data);
    let res = hmac.finalize();
    return res.into_bytes().to_vec();
}

fn expand_uni(passin :&[u8]) -> Vec<u8> {
    let mut retv :Vec<u8> = Vec::new();
    for i in 0..passin.len() {
        retv.push(0);
        retv.push(passin[i]);
    }
    /*at last one*/
    retv.push(0);
    retv.push(0);
    return retv;
}


fn pkcs12sha256_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let passin = ns.get_string("passin");
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    if sarr.len() < 1 {
        asn1obj_new_error!{Pkcs7Error,"need salt [datafile] [iterval] [defsize]"}
    }
    let salt :Vec<u8> = Vec::from_hex(&sarr[0]).unwrap();
    let  mut iterval :usize = 2048;
    let  mut defsize :usize = 32;

    if sarr.len() > 2 {
        iterval = sarr[2].parse::<usize>().unwrap();
    }
    if sarr.len() > 3 {
        defsize = sarr[3].parse::<usize>().unwrap();
    }

    let rdata = get_pkcs12kdf_sha256(passin.as_bytes(),&salt,PKCS12_MAC_ID,iterval,defsize);
    debug_buffer_trace!(rdata.as_ptr(),rdata.len(),"rdata");
    if sarr.len() > 1 {
        let data = read_file_bytes(&sarr[1])?;
        let digest = calc_hmac_sha256(&rdata,&data);
        debug_buffer_trace!(digest.as_ptr(),digest.len(),"digest");        
        let unidata = expand_uni(passin.as_bytes());
        let rdata = get_pkcs12kdf_sha256(&unidata,&salt,PKCS12_MAC_ID,iterval,defsize);
        debug_buffer_trace!(rdata.as_ptr(),rdata.len(),"uni rdata");
        let digest = calc_hmac_sha256(&rdata,&data);
        debug_buffer_trace!(digest.as_ptr(),digest.len(),"uni digest");        
    }

    Ok(())
}

fn check_equal_u8(a :&[u8],b :&[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }


    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn decode_pkcs12_code(code :&[u8],passin :&[u8]) -> Result<(),Box<dyn Error>> {
    let mut safes :Asn1AuthSafes = Asn1AuthSafes::init_asn1();
    let rlen = safes.decode_asn1(code)?;
    let mut f = std::io::stderr();
    debug_trace!("rlen [{}:0x{:x}]", rlen,rlen);
    let _ = safes.print_asn1("safes", 0, &mut f)?;
    let mut safeidx :usize= 0;

    /**/
    for idx in 0..safes.safes.val.len() {            
        let types = safes.safes.val[idx].elem.val[0].selector.val.get_value();
        //debug_trace!("types [{}]",types);
        if types == OID_PKCS7_ENCRYPTED_DATA {
            let pk7encdata :&Asn1Pkcs7Encrypt = safes.safes.val[idx].elem.val[0].encryptdata.val.as_ref().unwrap();
            let encdata = pk7encdata.elem.val[0].enc_data.elem.val[0].enc_data.val.data.clone();
            let algordata = pk7encdata.elem.val[0].enc_data.elem.val[0].algorithm.encode_asn1()?;
            let decdata = get_algor_pbkdf2_private_data(&algordata,&encdata,passin)?;
            let mut octdata :Asn1Seq<Asn1Pkcs12SafeBag> = Asn1Seq::init_asn1();
            let _ = octdata.decode_asn1(&decdata)?;
            let _ = octdata.print_asn1("safebag encdata", 0, &mut f)?;
            let mut certidx :usize = 0;
            for certd in octdata.val.iter() {
                let objs = certd.elem.val[0].selectelem.valid.val.get_value();
                if objs == OID_PKCS12_CERT_BAG {
                    let certtype = certd.elem.val[0].selectelem.bag.val[0].elem.val[0].valid.val.get_value();
                    if certtype == OID_PKCS12_SAFE_BAG_X509_CERT {
                        let certdata = certd.elem.val[0].selectelem.bag.val[0].elem.val[0].x509cert.val[0].data.clone();
                        let mut certp :Asn1X509 = Asn1X509::init_asn1();
                        let _ = certp.decode_asn1(&certdata)?;
                        let tagn = format!("safebag[{}]x509cert[{}]",safeidx,certidx);
                        let _ = certp.print_asn1(&tagn,0,&mut f)?;
                    } 
                } else if objs == OID_PKCS8_SHROUDED_KEY_BAG {
                    let x509sig :Asn1X509Sig = certd.elem.val[0].selectelem.shkeybag.val[0].clone();
                    let v8 = x509sig.encode_asn1()?;
                    let pkey = get_private_key(&v8,passin)?;
                    let kname = format!("safebag[{}]shroudbag cert[{}]", safeidx,certidx);
                    let _ = pkey.print_asn1(&kname, 0, &mut f)?;                        
                }
                certidx += 1;
            }

        } else if types ==  OID_PKCS7_DATA {
            let pk7data :&Asn1OctData = safes.safes.val[idx].elem.val[0].data.val.as_ref().unwrap();
            let decdata = pk7data.data.clone();
            let mut octdata :Asn1Seq<Asn1Pkcs12SafeBag> = Asn1Seq::init_asn1();
            let _ = octdata.decode_asn1(&decdata)?;
            let _ = octdata.print_asn1("safebag data", 0, &mut f)?;
            let mut bagidx :usize = 0;
            for bag in octdata.val.iter() {
                let objs = bag.elem.val[0].selectelem.valid.val.get_value();
                if objs == OID_PKCS8_SHROUDED_KEY_BAG {
                    let x509sig :Asn1X509Sig = bag.elem.val[0].selectelem.shkeybag.val[0].clone();
                    let v8 = x509sig.encode_asn1()?;
                    let pkey = get_private_key(&v8,passin)?;
                    let kname = format!("safebag[{}]shroudbag[{}]", safeidx,bagidx);
                    let _ = pkey.print_asn1(&kname, 0, &mut f)?;
                } else if objs == OID_PKCS12_CERT_BAG {
                    let certtype = bag.elem.val[0].selectelem.bag.val[0].elem.val[0].valid.val.get_value();
                    if certtype == OID_PKCS12_SAFE_BAG_X509_CERT {
                        let certdata = bag.elem.val[0].selectelem.bag.val[0].elem.val[0].x509cert.val[0].data.clone();
                        let mut certp :Asn1X509 = Asn1X509::init_asn1();
                        let _ = certp.decode_asn1(&certdata)?;
                        let tagn = format!("safebag[{}]x509cert bag[{}]",safeidx,bagidx);
                        let _ = certp.print_asn1(&tagn,0,&mut f)?;
                    }                        
                }
                bagidx += 1;
            }
        }
        safeidx += 1;
    }
    Ok(())
}

fn pkcs12dec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let passin = ns.get_string("passin");

    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let fname = format!("{}",f);
        let code = read_file_bytes(f)?;
        let mut xname = Asn1Pkcs12::init_asn1();
        let _ = xname.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        xname.print_asn1("pkcs12",0,&mut f)?;
        let macdata :&Asn1Pkcs12MacData = xname.elem.val[0].mac.val.as_ref().unwrap();
        let dinfo :Asn1X509Sig = macdata.elem.val[0].dinfo.clone();
        if dinfo.elem.val[0].algor.elem.val[0].algorithm.get_value() == OID_SHA256_DIGEST {
            let digest :Vec<u8> = dinfo.elem.val[0].digest.data.clone();
            let salt : Vec<u8> = macdata.elem.val[0].salt.data.clone();
            let iternum = macdata.elem.val[0].iternum.val;
            let hmac = get_pkcs12kdf_sha256(passin.as_bytes(),&salt,PKCS12_MAC_ID,iternum as usize,SHA256_DIGEST_SIZE);
            let chkd :&Asn1OctData = xname.elem.val[0].authsafes.elem.val[0].data.val.as_ref().unwrap();
            let chkdata = chkd.data.clone();
            let calcdigest = calc_hmac_sha256(&hmac,&chkdata);
            let mut checked :bool =false;
            if ! check_equal_u8(&calcdigest,&digest) {
                let unipass = expand_uni(passin.as_bytes());
                let hmac = get_pkcs12kdf_sha256(&unipass,&salt,PKCS12_MAC_ID,iternum as usize,SHA256_DIGEST_SIZE);
                let calcdigest = calc_hmac_sha256(&hmac,&chkdata);
                if check_equal_u8(&calcdigest,&digest) {
                    checked = true;
                }
            } else {
                checked = true;
            }
            if checked {
                let c = format!("{} Verify Ok\n", fname);
                let _ = f.write(c.as_bytes())?;
            } else {
                let c = format!("{} Verify Failed\n", fname);
                let _ = f.write(c.as_bytes())?;
            }
        }

        let types = xname.elem.val[0].authsafes.elem.val[0].selector.val.get_value();
        if types == OID_PKCS7_DATA {
            let p7data :&Asn1OctData = xname.elem.val[0].authsafes.elem.val[0].data.val.as_ref().unwrap();
            let code = p7data.data.clone();
            let _ = decode_pkcs12_code(&code,passin.as_bytes())?;
        }
    }

    Ok(())
}

fn authsafesdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let passin = ns.get_string("passin");
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let _ = decode_pkcs12_code(&code,passin.as_bytes())?;
    }
    Ok(())
}

fn pkcs12safebagdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    let passin = ns.get_string("passin");
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let code = read_file_bytes(f)?;
        let mut octdata :Asn1Seq<Asn1Pkcs12SafeBag> = Asn1Seq::init_asn1();
        let _ = octdata.decode_asn1(&code)?;
        let mut f = std::io::stderr();
        let _ = octdata.print_asn1("safebag", 0, &mut f)?;
        let mut idx :usize = 0;
        for bagv in octdata.val.iter() {
            let types = bagv.elem.val[0].selectelem.valid.val.get_value();
            if types == OID_PKCS8_SHROUDED_KEY_BAG {
                let x509sig :Asn1X509Sig = bagv.elem.val[0].selectelem.shkeybag.val[0].clone();
                let v8 = x509sig.encode_asn1()?;
                let pkey = get_private_key(&v8,passin.as_bytes())?;
                let _ = pkey.print_asn1("pkey", 0, &mut f)?;
            } else if types == OID_PKCS12_CERT_BAG {
                let certdata = bagv.elem.val[0].selectelem.bag.val[0].elem.val[0].x509cert.val[0].data.clone();
                let mut cert :Asn1X509 = Asn1X509::init_asn1();
                let _ = cert.decode_asn1(&certdata)?;
                let kname = format!("cert[{}]",idx);
                let _ = cert.print_asn1(&kname, 0, &mut f)?;
            } else {
                eprintln!("types [{}]", types);
            }
            idx += 1;
        }
    }
    Ok(())
}

#[allow(unused_assignments)]
fn decode_gpg_asc(s :&str) -> Result<(Vec<u8>,Vec<u8>), Box<dyn Error>> {
    let sarr :Vec<&str> = s.split("\n").collect();
    let mut maind :Vec<u8> = Vec::new();
    let mut bd :Vec<u8> = Vec::new();
    let mut c :String = "".to_string();
    for l in sarr.iter() {
        let mut kl = format!("{}",l);
        kl = kl.trim_end_matches("\r").to_string();
        if !kl.starts_with("---") {
            c.push_str(&kl);
            if kl.len() < 64 {
                if maind.len() == 0 {
                    maind = decode_base64(&c)?;
                    c = "".to_string();
                } else if bd.len() == 0 {
                    c = c[1..].to_string();
                    bd = decode_base64(&c)?;
                    c = "".to_string();
                }
            }
        }
    }

    if c.len() > 0 && bd.len() == 0 {
        c = c[1..].to_string();
        bd = decode_base64(&c)?;
        c = "".to_string();
    }
    return Ok((maind,bd));
}

fn decbase64_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    let input = ns.get_string("input");
    if input.len() > 0 {
        let s = read_file(&input)?;
        let v8 = decode_base64(&s)?;
        debug_buffer_trace!(v8.as_ptr(),v8.len(),"{} decode base64",input);
    } else {
        for f in sarr.iter() {
            let v8 = decode_base64(f)?;
            debug_buffer_trace!(v8.as_ptr(),v8.len(),"{} decode base64",f);
        }
    }
    Ok(())
}


fn gpgascdec_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {  
    let sarr :Vec<String>;
    init_log(ns.clone())?;
    sarr = ns.get_array("subnargs");
    for f in sarr.iter() {
        let s = read_file(f)?;
        let (maind,bd) = decode_gpg_asc(&s)?;
        debug_buffer_trace!(maind.as_ptr(),maind.len(),"maind for {}",f);
        debug_buffer_trace!(bd.as_ptr(),bd.len(), "bd for {}",f);
    }
    Ok(())
}




#[extargs_map_function(pkcs7dec_handler,x509namedec_handler,objenc_handler,pemtoder_handler,dertopem_handler,encryptprivdec_handler,privinfodec_handler,pbe2dec_handler,pbkdf2dec_handler,hmacsha256_handler,netpkeydec_handler,rsaprivdec_handler,x509dec_handler,sha256_handler,rsaverify_handler,csrdec_handler,pkcs12dec_handler,objdec_handler,authsafesdec_handler,pkcs12safebagdec_handler,pkcs12sha256_handler,rsapubdec_handler,gpgascdec_handler,decbase64_handler)]
pub fn load_pkcs7_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
    let cmdline = r#"
    {
        "passin" : null,
        "rsapriv"  : null,
        "capub" : null,
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
        "rsapubdec<rsapubdec_handler>##derfile ... to decode RSAPUBLICKEY##" : {
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
        },
        "pkcs12sha256<pkcs12sha256_handler>##salt  [datafile] [iterval] [retsize]  to format sha256 valid iterval default 2048 retsize default 32 ##" : {
            "$" : "+"
        },
        "gpgascdec<gpgascdec_handler>##gpgascfile ... to decode gpg file##" : {
            "$" : "+"
        },
        "decbase64<decbase64_handler>##str ... to decode base64##" : {
            "$" : "*"
        }
    }
    "#;
    extargs_load_commandline!(parser,cmdline)?;
    Ok(())
}