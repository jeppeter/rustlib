use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence};
use asn1obj::base::{Asn1Integer,Asn1Object,Asn1String,Asn1Any,Asn1ImpInteger,Asn1PrintableString};
use asn1obj::strop::{asn1_format_line};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::error::Error;

use std::io::{Write};


#[asn1_obj_selector(ci="1.2.3",co="1.2.5",cs="1.2.4",cs="1.2.21",ca=default)]
struct Asn1ObjSelector {
	pub val :Asn1Object,
}

#[asn1_choice()]
struct Asn1BB {
	pub selector : Asn1ObjSelector,
	pub ci :Asn1Integer,
	pub co :Asn1Object,
	pub cs :Asn1String,
	pub ca :Asn1Any,
}

#[asn1_sequence()]
struct Asn1Seqcc {
	pub v :Asn1ImpInteger<5>,
	pub s :Asn1String,
	pub o :Asn1Object,
}



fn main() {
	let mut av :Asn1BB = Asn1BB::init_asn1();
	let _ = av.selector.val.set_value("1.2.5").unwrap();
	let _ = av.co.set_value("2.5.7").unwrap();
	let code :Vec<u8> = av.encode_asn1().unwrap();
	println!("{:?}", code);
	let _ = av.selector.val.set_value("1.2.3").unwrap();
	av.ci.val = -20;
	let code :Vec<u8> = av.encode_asn1().unwrap();
	println!("{:?}", code);
	let _ = av.selector.val.set_value("1.2.4").unwrap();
	av.cs.val = format!("ccss222");
	let code :Vec<u8> = av.encode_asn1().unwrap();
	println!("{:?}", code);
	let _ = av.selector.val.set_value("1.2.21").unwrap();
	av.ca.tag = 0x12;
	av.ca.content = vec![0x44,0x22,0x43];
	let code :Vec<u8> = av.encode_asn1().unwrap();
	println!("{:?}", code);
	let _ = av.selector.val.set_value("1.2.27").unwrap();
	av.ca.tag = 0x12;
	av.ca.content = vec![0x44,0x22,0x43];
	let code :Vec<u8> = av.encode_asn1().unwrap();
	println!("{:?}", code);
	let mut va :Asn1Seqcc = Asn1Seqcc::init_asn1();
	va.v.val = 50;
	va.s.val = "value formed".to_string();
	let _ = va.o.set_value("1.2.55").unwrap();
	let code :Vec<u8> = va.encode_asn1().unwrap();
	println!("va code");
	println!("{:?}", code);
	
}
