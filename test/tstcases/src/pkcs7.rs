use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence};
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum};
use asn1obj::complex::{Asn1Set,Asn1SetOf,Asn1Seq};


#[asn1_sequence()]
struct Asn1X509Val {
	pub notBefore : Asn1Time,
	pub notAfter : Asn1Time,
}

#[asn1_sequence()]
struct Asn1X509Algor {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Opt<Asn1Any>,
}

#[asn1_sequence()]
struct Asn1Pkcs7Content {
	pub objval : Asn1Object,
	pub data :Asn1Any,	
}


#[asn1_sequence()]
struct Asn1RsaPubkey {
	pub n :Asn1BigNum,
	pub e :Asn1BigNum,
}

#[asn1_obj_selector(any=default,rsa="1.2.840.113549.1.1.1")]
struct Asn1X509PubkeySelector {
	pub val : Asn1Object,
}

#[asn1_choice(selector=valid)]
struct Asn1X509Pubkey {
	pub valid : Asn1X509PubkeySelector,
	pub rsa : Asn1Seq<Asn1RsaPubkey>,
	pub any : Asn1Any,
}

#[asn1_sequence()]
struct Asn1X509Cinf {
	pub version : Asn1Opt<Asn1ImpEncap<Asn1Integer,0>>,
	pub serial_number :Asn1Integer,
	pub signature : Asn1X509Algor,
	pub issuer : Asn1X509Name,
	pub validity : Asn1X509Val,
	pub subject :Asn1X509Name,
}

#[asn1_sequence()]
struct Asn1X509 {
	pub certinfo : Asn1X509Cinf,
	pub sig_alg : Asn1X509Algor,
	pub signature : Asn1BitString,
}

#[asn1_sequence()]
struct Asn1Pkcs7Signed {
	pub version :Asn1Integer,
	pub md_algs : Asn1Set<Asn1X509Algor>,
	pub contents : Asn1Pkcs7Content,
	pub cert :Asn1Opt<Asn1SetOf<Asn1X509,0>>,
}


#[asn1_obj_selector(anyobj=default,signed="1.2.840.113549.1.7.2")]
struct Asn1Pkcs7Selector {
	pub val :Asn1Object,
}

#[asn1_choice(selector=selector)]
struct Asn1Pkcs7 {
	pub selector :Asn1Pkcs7Selector,
	pub signed :Asn1Pkcs7Signed,
	pub anyobj :Asn1Any,
}