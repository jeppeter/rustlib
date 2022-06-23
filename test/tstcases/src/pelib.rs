use pelite::pe32::PeFile as pe32file;
use pelite::pe64::PeFile as pe64file;
use pelite::pe32::Pe as pe32obj;
use pelite::pe64::Pe as pe64obj;
use pelite::{FileMap};
use pelite::image::{IMAGE_DIRECTORY_ENTRY_SECURITY};

use std::error::Error;

#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};


#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};

extargs_error_class!{PeLibError}

pub struct SecData {
	pub buf :Vec<u8>,
	pub size :u32,
	pub virtaddr :u32,
}


pub fn get_securtiy_buffer(fname :&str) -> Result<SecData,Box<dyn Error>> {
	let fomap = FileMap::open(fname);
	let mut retv :Vec<u8> = Vec::new();
	let sdata :SecData;
	let virtaddr :u32;
	let mut vsize :u32;
	if fomap.is_err() {
		let err = fomap.err().unwrap();
		extargs_new_error!{PeLibError,"can not load [{}] error [{:?}]", fname,err}
	}
	let fmap = fomap.unwrap();
	let fo64 = pe64file::from_bytes(&fmap);
	let fo32;
	if fo64.is_err() {
		fo32 = pe32file::from_bytes(&fmap);
		if fo32.is_err() {
			let err = fo32.err().unwrap();
			extargs_new_error!{PeLibError,"can not parse [{}] from bytes error[{:?}]",fname,err}			
		}
		let file = fo32.unwrap();
		let sodata = file.data_directory().get(IMAGE_DIRECTORY_ENTRY_SECURITY);
		if sodata.is_none() {
			extargs_new_error!{PeLibError,"[{}] no IMAGE_DIRECTORY_ENTRY_SECURITY", fname}
		}
		let secdata = sodata.unwrap();
		virtaddr = secdata.VirtualAddress;
		vsize = secdata.Size;
		let mut i :u32 =0;
		while i < vsize {
			let odata = file.slice_bytes(secdata.VirtualAddress + i);
			if odata.is_err() {
				break;
			}

			let data = odata.unwrap();
			for b in data {
				retv.push(*b);
				i += 1;
			}
		}

		let sections = file.section_headers().as_slice();
		for s in sections.iter() {
			if s.VirtualAddress < virtaddr && virtaddr < (s.VirtualAddress + s.VirtualSize) {
				if vsize > (s.VirtualAddress + s.VirtualSize - virtaddr) {
					vsize = s.VirtualAddress + s.VirtualSize - virtaddr;
				}
				break;
			}
		}
	} else {
		let file = fo64.unwrap();
		let sodata = file.data_directory().get(IMAGE_DIRECTORY_ENTRY_SECURITY);
		if sodata.is_none() {
			extargs_new_error!{PeLibError,"[{}] no IMAGE_DIRECTORY_ENTRY_SECURITY", fname}
		}
		let secdata = sodata.unwrap();
		virtaddr = secdata.VirtualAddress;
		vsize = secdata.Size;
		let mut i :u32 =0;
		while i < vsize {
			let odata = file.slice_bytes(secdata.VirtualAddress + i);
			if odata.is_err() {
				break;
			}

			let data = odata.unwrap();
			for b in data {
				retv.push(*b);
				i += 1;
			}
		}

		let sections = file.section_headers().as_slice();
		for s in sections.iter() {
			if s.VirtualAddress < virtaddr && virtaddr < (s.VirtualAddress + s.VirtualSize) {
				if vsize > (s.VirtualAddress + s.VirtualSize - virtaddr) {
					vsize = s.VirtualAddress + s.VirtualSize - virtaddr;
				}
				break;
			}
		}
	}
		
	sdata = SecData {
		virtaddr : virtaddr,
		size :vsize,
		buf :retv.clone(),
	};
	Ok(sdata)
}