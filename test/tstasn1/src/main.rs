use asn1obj_codegen::{asn1_choice,asn1_obj_selector};
use asn1obj::base::{Asn1Integer,Asn1Object,Asn1String};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::error::Error;

use std::io::{Write};


#[asn1_obj_selector(ci="1.2.3",co="1.2.5",cs="1.2.4")]
struct Asn1ObjSelector {
	pub val :Asn1Object,
}





#[asn1_choice()]
struct Asn1BB {
	pub selector : Asn1ObjSelector,
	pub ci :Asn1Integer,
	pub co :Asn1Object,
	pub cs :Asn1String,
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
}
