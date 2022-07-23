use asn1obj_codegen::{asn1_choice,asn1_selector};
use asn1obj::base::{Asn1Integer,Asn1Object,Asn1String};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};


#[asn1_selector(1.2.3=ci,1.2.5=co,1.2.4=cs)]
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
	println!("hello world");
}
