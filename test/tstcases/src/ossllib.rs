#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_type_choice};
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1PrintableString,Asn1BitString,Asn1Null,Asn1OctData,Asn1BitData};
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
use asn1obj::strop::{asn1_format_line};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::error::Error;
use std::boxed::Box;
use std::io::{Write};

#[asn1_type_choice(debug=3,unicode=2,ascii=1,selector=stype)]
pub struct SpcString {
	pub stype :i32,
	pub unicode : Asn1Imp<Asn1BitString,0>,
	pub ascii :Asn1Imp<Asn1BitString,1>,
}