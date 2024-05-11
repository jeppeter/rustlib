
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use std::io;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use sha2::{Sha256,Digest};


use std::error::Error;

extargs_error_class!{FileOpError}

#[allow(dead_code)]
pub fn write_file_bytes(fname :&str, byts :&[u8]) -> Result<(),Box<dyn Error>> {
	if fname.len() == 0 {
		let res = io::stdout().write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}	
		}
	} else {
		let fo  = fs::File::create(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"create [{}] error[{:?}]", fname,err}
		}
		let mut fp :fs::File = fo.unwrap();
		let res = fp.write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"write [{}] len[{}] error[{:?}]", fname, byts.len(),err}	
		}
	}
	Ok(())
}

#[allow(dead_code)]
pub fn write_file(fname :&str, outs :&str) -> Result<(),Box<dyn Error>> {
	return write_file_bytes(fname,outs.as_bytes());
}


#[allow(dead_code)]
pub fn read_file_bytes(fname :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(buf)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(buf)		
	}
}

#[allow(dead_code)]
pub fn read_file(fname :&str) -> Result<String,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = io::stdin();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(retv)
	} else {
		let fo = fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			extargs_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = BufReader::new(f);
		let mut retv :String = String::new();
		let res = reader.read_to_string(&mut retv);
		if res.is_err() {
			let err = res.err().unwrap();
			extargs_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(retv)		
	}
}

#[allow(dead_code)]
pub fn get_sha256_data(ind :&[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(&ind);
    let res = hasher.finalize();
    return res.to_vec();    
}

pub fn touch_file(infile :&str) -> Result<(),Box<dyn Error>> {
	let fpath = std::path::Path::new(infile);
	if !fpath.exists() {
		match std::fs::OpenOptions::new().create(true).write(true).open(&fpath) {
			Ok(_) => {
				return Ok(());
			},
			Err(e) => {
				extargs_new_error!{FileOpError,"touch {} error {:?}",infile,e}
			}
		}		
	}
	Ok(())
}


pub fn delete_file(infile :&str) -> Result<(),Box<dyn Error>> {
	let fpath = std::path::Path::new(infile);
	if fpath.exists() {
		let ores = std::fs::remove_file(&fpath);
		if ores.is_err() {
			extargs_new_error!{FileOpError,"remove file [{}] error {:?}",infile,ores.err().unwrap()}
		}
	}
	Ok(())
}

pub fn exists_file(infile :&str) -> bool {
	let fpath = std::path::Path::new(infile);
	if fpath.exists() {
		return true;
	}
	return false;
}