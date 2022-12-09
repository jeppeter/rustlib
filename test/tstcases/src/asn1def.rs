#[allow(unused_imports)]
use asn1obj_codegen::{asn1_choice,asn1_obj_selector,asn1_sequence,asn1_int_choice};
#[allow(unused_imports)]
use asn1obj::base::{Asn1Object,Asn1Integer,Asn1BigNum,Asn1Any,Asn1Time,Asn1Boolean,Asn1PrintableString,Asn1BitString,Asn1Null,Asn1OctData,Asn1BitData,Asn1IA5String};
use asn1obj::complex::{Asn1Set,Asn1ImpSet,Asn1Seq,Asn1Opt,Asn1Imp,Asn1Ndef,Asn1SeqSelector,Asn1BitSeq};
use asn1obj::strop::{asn1_format_line};
use asn1obj::asn1impl::{Asn1Op,Asn1Selector};
#[allow(unused_imports)]
use asn1obj::{asn1obj_error_class,asn1obj_new_error};

use std::error::Error;
use std::boxed::Box;
use std::io::{Write};
use std::cmp::PartialEq;
#[allow(unused_imports)]
use super::{debug_trace,debug_buffer_trace,format_buffer_log,debug_error,format_str_log};
#[allow(unused_imports)]
use super::loglib::{log_get_timestamp,log_output_function,init_log};
#[allow(unused_imports)]
use super::fileop::{read_file,read_file_bytes,write_file_bytes,get_sha256_data};
#[allow(unused_imports)]
use super::pemlib::{pem_to_der,der_to_pem};
#[allow(unused_imports)]
use rsa::{RsaPublicKey,RsaPrivateKey,PublicKey};
use rsa::BigUint as rsaBigUint;
use rsa::hash::{Hash};
use rsa::padding::{PaddingScheme};

use hmac::{Hmac,Mac};
#[allow(unused_imports)]
use sha2::{Sha256,Digest};
use super::cryptlib::{aes256_cbc_decrypt};


pub const OID_PBES2 :&str = "1.2.840.113549.1.5.13";
pub const OID_PBKDF2 :&str = "1.2.840.113549.1.5.12";
pub const OID_AES_256_CBC :&str = "2.16.840.1.101.3.4.1.42";
pub const OID_RSA_ENCRYPTION :&str = "1.2.840.113549.1.1.1";
pub const OID_SHA256_WITH_RSA_ENCRYPTION :&str = "1.2.840.113549.1.1.11";
pub const OID_PKCS8_SHROUDED_KEY_BAG :&str = "1.2.840.113549.1.12.10.1.2";
pub const OID_PKCS12_CERT_BAG : &str = "1.2.840.113549.1.12.10.1.3";
pub const OID_PKCS7_ENCRYPTED_DATA :&str = "1.2.840.113549.1.7.6";
pub const OID_PKCS7_DATA :&str = "1.2.840.113549.1.7.1";
pub const OID_PKCS12_SAFE_BAG_X509_CERT :&str = "1.2.840.113549.1.9.22.1";
pub const OID_SHA256_DIGEST :&str = "2.16.840.1.101.3.4.2.1";
pub const OID_SHA256_DIGEST_SET :&str = "1.2.840.113549.1.9.4";



asn1obj_error_class!{Asn1DefError}

pub trait Asn1DigestOp {
	fn digest(&self, data :&[u8]) -> Result<Vec<u8>,Box<dyn Error>>;
}

pub struct Sha256Digest {	
}

impl Sha256Digest {
	fn calc(data :&[u8]) -> Vec<u8> {
		let retv = get_sha256_data(data);
		retv
	}	
}

impl Asn1DigestOp for Sha256Digest {
	fn digest(&self, data :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
		let retv = Self::calc(data);
		Ok(retv)
	}
}

impl Sha256Digest {
	pub fn new() -> Self {
		Sha256Digest{}
	}
}

pub trait Asn1SignOp {
	fn sign(&self,data :&[u8],digop :Box<dyn Asn1DigestOp>) -> Result<Vec<u8>,Box<dyn Error>>;
}

pub trait Asn1VerifyOp {
	fn verify(&self, origdata :&[u8],signdata :&[u8], digop :Box<dyn Asn1DigestOp>) -> Result<bool,Box<dyn Error>>;
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509NameElement {
	pub obj : Asn1Object,
	pub name :Asn1PrintableString,
}

impl Asn1X509NameElement {
	pub fn format_name(&self) -> String {
		let rets :String;
		rets = format!("{}:{}",self.obj.get_value(),self.name.val);
		return rets;
	}
}


//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509NameEntry {
	pub names : Asn1Set<Asn1Seq<Asn1X509NameElement>>,
}


impl Asn1X509NameEntry {
	pub fn get_names(&self) -> Vec<String>{
		let mut retn :Vec<String> = Vec::new();
		for v in self.names.val.iter() {
			for bv in v.val.iter() {
				retn.push(bv.format_name());
			}
		}
		return retn;
	}
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Name {
	pub entries : Asn1Seq<Asn1X509NameEntry>,
}

impl  PartialEq for Asn1X509Name {

	fn ne(&self,other :&Self) -> bool {
		let snames :Vec<String>;
		let onames :Vec<String>;
		let mut bmatched :bool;

		if self.entries.val.len() == 0 && other.entries.val.len() == 0 {
			return false;
		} else if self.entries.val.len() == 0 {
			return true;
		} else if other.entries.val.len() == 0 {
			return true;
		} else {
			snames = self.entries.val[0].get_names();
			onames = other.entries.val[0].get_names();
			if snames.len() == 0 && onames.len() == 0 {
				return false;
			} else if snames.len() == 0 {
				return true;
			} else if onames.len() == 0 {
				return true;
			}
			for i in 0..snames.len() {
				bmatched = false;
				for j in 0..onames.len() {
					if snames[i].eq(&(onames[j])) {
						bmatched = true;
						break;
					}
				}

				if !bmatched {
					return true;
				}
			}

			for j in 0..onames.len() {
				bmatched = false;
				for i in 0..snames.len() {
					if onames[j].eq(&snames[i]) {
						bmatched = true;
						break;
					}
				}
				if !bmatched {
					return true;
				}
			}
		}
		return false;
	}

	fn eq(&self, other :&Self) -> bool {
		if self.ne(other) {
			return false;
		}
		return true;
	}

}


//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509AttributeElem {
	pub object :Asn1Object,
	pub set :Asn1Any,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Attribute {
	pub elem : Asn1Seq<Asn1X509AttributeElem>,
}

impl Asn1X509Attribute {
	pub fn check_set_object_val(&mut self,objval :&Asn1Object,setval :&Asn1Any) -> Result<bool,Box<dyn Error>> {
		let mut retv :bool = false;
		if self.elem.val.len() != 0 && self.elem.val.len()!=1 {
			asn1obj_new_error!{Asn1DefError,"val [{}] != 0 or 1",self.elem.val.len()}
		}
		if self.elem.val.len() != 0 {
			if self.elem.val[0].object.eq(objval) {
				self.elem.val[0].set = setval.clone();
				retv= true;
			}
		}
		Ok(retv)
	}
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509ValElem {
	pub notBefore : Asn1Time,
	pub notAfter : Asn1Time,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Val {
	pub elem : Asn1Seq<Asn1X509ValElem>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509AlgorElem {
	pub algorithm : Asn1Object,
	pub parameters : Asn1Opt<Asn1Any>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Algor {
	pub elem : Asn1Seq<Asn1X509AlgorElem>,
}


//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7ContentElem {
	pub objval : Asn1Object,
	pub data :Asn1Opt<Asn1Any>,	
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7Content {
	pub elem :Asn1Seq<Asn1Pkcs7ContentElem>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPubkeyElem {
	pub n :Asn1BigNum,
	pub e :Asn1BigNum,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPubkey {
	pub elem :Asn1Seq<Asn1RsaPubkeyElem>,
}

impl Asn1VerifyOp for Asn1RsaPubkey {
	fn verify(&self, origdata :&[u8],signdata :&[u8], digop :Box<dyn Asn1DigestOp>) -> Result<bool,Box<dyn Error>> {
		let mut retv :bool = false;
		if self.elem.val.len() != 1 {
			asn1obj_new_error!{Asn1DefError,"{} != 1 len",self.elem.val.len()}
		}
		let n = rsaBigUint::from_bytes_be(&self.elem.val[0].n.val.to_bytes_be());
		let e = rsaBigUint::from_bytes_be(&self.elem.val[0].e.val.to_bytes_be());
		let pubk = RsaPublicKey::new(n,e)?;
		let digest = digop.digest(origdata)?;
		let ores = pubk.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,signdata);
		if ores.is_ok() {
			retv = true;
		} 
		Ok(retv)
	}
}

//#[asn1_obj_selector(selector=val,any=default,rsa="1.2.840.113549.1.1.1",debug=enable)]
#[asn1_obj_selector(selector=val,any=default,rsa="1.2.840.113549.1.1.1")]
#[derive(Clone)]
pub struct Asn1X509PubkeySelector {
	pub val : Asn1Object,
	pub padded : Asn1Any,
}

//#[asn1_choice(selector=valid,debug=enable)]
#[asn1_choice(selector=valid)]
#[derive(Clone)]
pub struct Asn1X509PubkeyElem {
	pub valid : Asn1SeqSelector<Asn1X509PubkeySelector>,
	pub rsa : Asn1BitSeq<Asn1RsaPubkey>,
	pub any : Asn1Any,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Pubkey {
	pub elem :Asn1Seq<Asn1X509PubkeyElem>,
}
//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509ExtensionElem {
	pub object :Asn1Object,
	pub critical : Asn1Opt<Asn1Boolean>,
	pub value : Asn1OctData,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Extension {
	pub elem :Asn1Seq<Asn1X509ExtensionElem>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509CinfElem {
	pub version : Asn1Opt<Asn1ImpSet<Asn1Integer,0>>,
	pub serial_number :Asn1BigNum,
	pub signature : Asn1X509Algor,
	pub issuer : Asn1X509Name,
	pub validity : Asn1X509Val,
	pub subject :Asn1X509Name,
	pub key : Asn1X509Pubkey,
	pub issuerUID : Asn1Opt<Asn1Imp<Asn1BitString,1>>,
	pub subjectUID : Asn1Opt<Asn1Imp<Asn1BitString,2>>,
	pub extensions : Asn1Opt<Asn1ImpSet<Asn1Seq<Asn1X509Extension>,3>>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Cinf {
	pub elem : Asn1Seq<Asn1X509CinfElem>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Revoked {
	pub serialNumber : Asn1Integer,
	pub revocationDate : Asn1Time,
	pub extensions : Asn1Opt<Asn1Seq<Asn1X509Extension>>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509CrlInfo {
	pub version : Asn1Opt<Asn1Integer>,
	pub sig_alg : Asn1X509Algor,
	pub issuer : Asn1X509Name,
	pub lastUpdate : Asn1Time,
	pub nextUpdate :Asn1Time,
	pub revoked : Asn1Opt<Asn1Seq<Asn1X509Revoked>>,
	pub extensions : Asn1Opt<Asn1Seq<Asn1X509Extension>>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Crl {
	pub crl : Asn1X509CrlInfo,
	pub sig_alg :Asn1X509Algor,
	pub signature : Asn1BitString,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Elem {
	pub certinfo : Asn1X509Cinf,
	pub sig_alg : Asn1X509Algor,
	pub signature : Asn1BitData,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509 {
	pub elem : Asn1Seq<Asn1X509Elem>,
}

impl Asn1X509 {
	pub fn is_self_signed(&self) -> bool {
		if self.elem.val.len() != 1 {
			debug_error!("{} len != 1" ,self.elem.val.len());
			return false;
		}
		let certinfo :&Asn1X509Cinf = &self.elem.val[0].certinfo;

		if certinfo.elem.val.len() != 1 {
			debug_error!("certinfo {} len != 1" ,certinfo.elem.val.len());
			return false;
		}

		if certinfo.elem.val[0].issuer.eq(&certinfo.elem.val[0].subject) {
			return true;
		}

		return false;
	}
}


//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7IssuerAndSerialElem {
	pub issuer : Asn1X509Name,
	pub serial : Asn1BigNum,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7IssuerAndSerial {
	pub elem :Asn1Seq<Asn1Pkcs7IssuerAndSerialElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509AttrPack {
	pub elem :Asn1Set<Asn1X509Attribute>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7SignerInfoElem {
	pub version : Asn1Integer,
	pub issuer_and_serial : Asn1Pkcs7IssuerAndSerial,
	pub digest_algo : Asn1X509Algor,
	pub auth_attr : Asn1Opt<Asn1ImpSet<Asn1X509Attribute,0>>,
	pub digest_enc_algo : Asn1X509Algor,
	pub enc_digest : Asn1OctData,
	pub unauth_attr : Asn1Opt<Asn1ImpSet<Asn1X509Attribute,1>>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7SignerInfo {
	pub elem : Asn1Seq<Asn1Pkcs7SignerInfoElem>,
}

impl Asn1Pkcs7SignerInfo {
	pub fn get_auth_attrs(&self) -> Result<Vec<Asn1X509Attribute>,Box<dyn Error>> {
		let mut retv :Vec<Asn1X509Attribute> = Vec::new();
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			asn1obj_new_error!{Asn1DefError,"val [{}] != 0 or 1",self.elem.val.len()}
		}

		if self.elem.val.len() == 1 {
			if self.elem.val[0].auth_attr.val.is_some() {
				let cset :&Asn1ImpSet<Asn1X509Attribute,0> = self.elem.val[0].auth_attr.val.as_ref().unwrap();
				for k in cset.val.iter() {
					retv.push(k.clone());
				}
			}
		}

		Ok(retv)
	}

	pub fn set_auth_attrs(&mut self, attrs :&Vec<Asn1X509Attribute>) -> Result<(),Box<dyn Error>> {
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			asn1obj_new_error!{Asn1DefError,"val [{}] != 0 or 1",self.elem.val.len()}	
		}

		if self.elem.val.len() == 0 {
			self.elem.val.push(Asn1Pkcs7SignerInfoElem::init_asn1());
		}

		if attrs.len() == 0 {
			self.elem.val[0].auth_attr.val = None;
		} else {
			let mut cset :Asn1ImpSet<Asn1X509Attribute,0> = Asn1ImpSet::init_asn1();
			cset.val = attrs.clone();
			self.elem.val[0].auth_attr.val = Some(cset);
		}
		Ok(())
	}

	fn format_auth_attr_data(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let mut attrs :Asn1X509AttrPack = Asn1X509AttrPack::init_asn1();
		if self.elem.val[0].auth_attr.val.is_some() {
			let c = self.elem.val[0].auth_attr.val.as_ref().unwrap();

			for k in c.val.iter() {
				attrs.elem.val.push(k.clone());
			}
		}
		let data = attrs.encode_asn1()?;
		Ok(data)
	}

	fn get_digest_op(&self) -> Box<dyn Asn1DigestOp> {
		let mut retv :Box<dyn Asn1DigestOp> = Box::new(Sha256Digest::new());

		if self.elem.val[0].digest_algo.elem.val.len() > 0 {
			let c = &(self.elem.val[0].digest_algo.elem.val[0]);
			let digval :String = c.algorithm.get_value();
			if digval.eq(OID_SHA256_DIGEST) {
				retv = Box::new(Sha256Digest::new());
			}
		}


		retv
	}

	pub fn sign_auth_attr_enc<T : Asn1SignOp>(&mut self, signer :&T) -> Result<(),Box<dyn Error>> {
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			asn1obj_new_error!{Asn1DefError,"val [{}] != 0 or 1",self.elem.val.len()}	
		}
		if self.elem.val.len() != 0 {
			let encdata = self.format_auth_attr_data()?;
			debug_buffer_trace!(encdata.as_ptr(),encdata.len(),"sign data");
			let digop = self.get_digest_op();
			self.elem.val[0].enc_digest.data = signer.sign(&encdata,digop)?;
		}
		Ok(())
	}

}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7SignedElem {
	pub version :Asn1Integer,
	pub md_algs : Asn1Set<Asn1X509Algor>,
	pub contents : Asn1Pkcs7Content,
	pub cert :Asn1Opt<Asn1ImpSet<Asn1X509,0>>,
	pub crl : Asn1ImpSet<Asn1X509Crl,1>,
	pub signer_info : Asn1Set<Asn1Pkcs7SignerInfo>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7Signed {
	pub elem : Asn1Seq<Asn1Pkcs7SignedElem>,
}


impl Asn1Pkcs7Signed {
	pub fn get_certs(&self) -> Result<Vec<Asn1X509>,Box<dyn Error>> {
		let mut retv :Vec<Asn1X509> = Vec::new();
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			asn1obj_new_error!{Asn1DefError,"elem [{}] not valid", self.elem.val.len()}
		}
		if self.elem.val.len() > 0 {
			if self.elem.val[0].cert.val.is_some() {
				let b = self.elem.val[0].cert.val.as_ref().unwrap();
				for v in b.val.iter() {
					let code = v.encode_asn1()?;
					let mut cv :Asn1X509 = Asn1X509::init_asn1();
					let _ = cv.decode_asn1(&code)?;
					retv.push(cv);
				}
			}
		}
		Ok(retv)
	}
	pub fn set_certs(&mut self, certs :&Vec<Asn1X509>) -> Result<(),Box<dyn Error>> {
		let mut cimp :Asn1ImpSet<Asn1X509,0> = Asn1ImpSet::init_asn1();
		cimp.val = certs.clone();
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			asn1obj_new_error!{Asn1DefError,"elem [{}] not valid",self.elem.val.len()}
		}
		if self.elem.val.len() == 0 {
			let c = Asn1Pkcs7SignedElem::init_asn1();
			self.elem.val.push(c);
		}
		self.elem.val[0].cert.val = Some(cimp);
		return Ok(());
	}

	pub fn get_signer_info_mut(&mut self,i :usize) -> Option<&mut Asn1Pkcs7SignerInfo> {
		if self.elem.val.len() != 1 && self.elem.val.len() != 0 {
			return None;
		}

		if self.elem.val.len() != 0 {
			if i < self.elem.val[0].signer_info.val.len() {
				return Some(&mut self.elem.val[0].signer_info.val[i]);
			}
		}
		return None;
	}


}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7EncContentElem {
	pub content_type : Asn1Object,
	pub algorithm : Asn1X509Algor,
	pub enc_data :Asn1Imp<Asn1OctData,0>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7EncContent {
	pub elem :Asn1Seq<Asn1Pkcs7EncContentElem>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7EncryptElem {
	pub version : Asn1Integer,
	pub enc_data : Asn1Pkcs7EncContent,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7Encrypt {
	pub elem : Asn1Seq<Asn1Pkcs7EncryptElem>,
}

//#[asn1_obj_selector(debug=enable,anyobj=default,signed="1.2.840.113549.1.7.2",encryptdata="1.2.840.113549.1.7.6",data="1.2.840.113549.1.7.1")]
#[asn1_obj_selector(anyobj=default,signed="1.2.840.113549.1.7.2",encryptdata="1.2.840.113549.1.7.6",data="1.2.840.113549.1.7.1")]
#[derive(Clone)]
pub struct Asn1Pkcs7Selector {
	pub val :Asn1Object,
}

//#[asn1_choice(selector=selector,debug=enable)]
#[asn1_choice(selector=selector)]
#[derive(Clone)]
pub struct Asn1Pkcs7Elem {
	pub selector :Asn1Pkcs7Selector,
	pub signed : Asn1Ndef<Asn1Pkcs7Signed,0>,
	pub encryptdata : Asn1Ndef<Asn1Pkcs7Encrypt,0>,
	pub data : Asn1Ndef<Asn1OctData,0>,
	pub anyobj :Asn1Any,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs7 {
	pub elem :Asn1Seq<Asn1Pkcs7Elem>,
}

#[allow(dead_code)]
impl Asn1Pkcs7 {
	pub fn is_signed_data(&self) -> bool {
		if self.elem.val.len() < 1 {
			return false;
		}
		let ores = self.elem.val[0].selector.encode_select();
		if ores.is_err() {
			return false;
		}
		let val = ores.unwrap();
		if val == "signed" {
			return true;
		}
		return false;
	}

	pub fn get_signed_data(&self) -> Result<&Asn1Pkcs7Signed,Box<dyn Error>> {
		if self.is_signed_data() {
			let p = self.elem.val[0].signed.val.as_ref().unwrap();
			return Ok(p);
		}
		asn1obj_new_error!{Asn1DefError,"not signed data"}
	}

	pub fn get_signed_data_mut(&mut self) -> Result<&mut Asn1Pkcs7Signed,Box<dyn Error>> {
		if self.is_signed_data() {
			return Ok(self.elem.val[0].signed.val.as_mut().unwrap());
		}
		asn1obj_new_error!{Asn1DefError,"not signed data"}	
	}
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1AuthSafes {
	pub safes :Asn1Seq<Asn1Pkcs7>,
}
//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs8PrivKeyInfoElem {
	pub version :Asn1Integer,
	pub pkeyalg : Asn1X509Algor,
	pub pkey : Asn1OctData,
	pub attributes : Asn1Opt<Asn1ImpSet<Asn1X509Attribute,0>>,
}

//#[asn1_sequence(debug=enable)]
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs8PrivKeyInfo {
	pub elem : Asn1Seq<Asn1Pkcs8PrivKeyInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509SigElem {
	pub algor : Asn1X509Algor,
	pub digest : Asn1OctData,
}


#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Sig {
	pub elem : Asn1Seq<Asn1X509SigElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pbe2ParamElem {
	pub keyfunc : Asn1X509Algor,
	pub encryption : Asn1X509Algor,
}


#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pbe2Param {
	pub elem : Asn1Seq<Asn1Pbe2ParamElem>,
}
#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pbkdf2ParamElem {
	pub salt : Asn1Any,
	pub iter : Asn1Integer,
	pub keylength :Asn1Opt<Asn1Integer>,
	pub prf : Asn1Opt<Asn1X509Algor>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pbkdf2Param {
	pub elem : Asn1Seq<Asn1Pbkdf2ParamElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1NetscapePkeyElem {
	pub version :Asn1Integer,
	pub algor : Asn1X509Algor,
	pub privdata :Asn1OctData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1NetscapePkey {
	pub elem : Asn1Seq<Asn1NetscapePkeyElem>,
}


#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPrivateKeyElem {
	pub version :Asn1Integer,
	pub modulus : Asn1BigNum,
	pub pubexp : Asn1BigNum,
	pub privexp : Asn1BigNum,
	pub prime1 :Asn1BigNum,
	pub prime2 :Asn1BigNum,
	pub exp1 : Asn1BigNum,
	pub exp2 :Asn1BigNum,
	pub coeff : Asn1BigNum,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPrivateKey {
	pub elem : Asn1Seq<Asn1RsaPrivateKeyElem>,
}

impl Asn1SignOp for Asn1RsaPrivateKey {
	fn sign(&self,data :&[u8],digop :Box<dyn Asn1DigestOp>) -> Result<Vec<u8>,Box<dyn Error>> {
		let retv :Vec<u8>;
		if self.elem.val.len() != 1 {
			asn1obj_new_error!{Asn1DefError,"{} not valid len",self.elem.val.len()}
		}

		let n = rsaBigUint::from_bytes_be(&self.elem.val[0].modulus.val.to_bytes_be());
		let d = rsaBigUint::from_bytes_be(&self.elem.val[0].pubexp.val.to_bytes_be());
		let e = rsaBigUint::from_bytes_be(&self.elem.val[0].privexp.val.to_bytes_be());
		let mut primes :Vec<rsaBigUint> = Vec::new();
		primes.push(rsaBigUint::from_bytes_be(&self.elem.val[0].prime1.val.to_bytes_be()));
		primes.push(rsaBigUint::from_bytes_be(&self.elem.val[0].prime2.val.to_bytes_be()));
		let po = RsaPrivateKey::from_components(n,d,e,primes);
		let digest = digop.digest(data)?;
		retv = po.sign(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest)?;
		debug_buffer_trace!(retv.as_ptr(),retv.len(),"sign value");
		Ok(retv)
	}
}

impl Asn1VerifyOp for Asn1RsaPrivateKey {
	fn verify(&self, origdata :&[u8],signdata :&[u8], digop :Box<dyn Asn1DigestOp>) -> Result<bool,Box<dyn Error>> {
		let mut retv :bool = false;
		if self.elem.val.len() != 1 {
			asn1obj_new_error!{Asn1DefError,"{} != 1 len",self.elem.val.len()}
		}
		let n = rsaBigUint::from_bytes_be(&self.elem.val[0].modulus.val.to_bytes_be());
		let d = rsaBigUint::from_bytes_be(&self.elem.val[0].pubexp.val.to_bytes_be());
		let e = rsaBigUint::from_bytes_be(&self.elem.val[0].privexp.val.to_bytes_be());
		let mut primes :Vec<rsaBigUint> = Vec::new();
		primes.push(rsaBigUint::from_bytes_be(&self.elem.val[0].prime1.val.to_bytes_be()));
		primes.push(rsaBigUint::from_bytes_be(&self.elem.val[0].prime2.val.to_bytes_be()));
		let po = RsaPrivateKey::from_components(n,d,e,primes);
		let pubk = po.to_public_key();
		let digest = digop.digest(origdata)?;
		let ores = pubk.verify(PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256)),&digest,signdata);
		if ores.is_ok() {
			retv = true;
		} 
		Ok(retv)
	}	
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509ReqInfoElem {
	pub version : Asn1Integer,
	pub subject : Asn1X509Name,
	pub pubkey : Asn1X509Pubkey,
	pub attributes : Asn1Opt<Asn1ImpSet<Asn1X509Attribute,0>>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509ReqInfo {
	pub elem : Asn1Seq<Asn1X509ReqInfoElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509ReqElem {
	pub req_info : Asn1X509ReqInfo,
	pub sig_alg : Asn1X509Algor,
	pub signature : Asn1BitData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1X509Req {
	pub elem : Asn1Seq<Asn1X509ReqElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12MacDataElem {
	pub dinfo : Asn1X509Sig,
	pub salt : Asn1OctData,
	pub iternum : Asn1Integer,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12MacData {
	pub elem : Asn1Seq<Asn1Pkcs12MacDataElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12Elem {
	pub version : Asn1Integer,
	pub authsafes : Asn1Pkcs7,
	pub mac : Asn1Opt<Asn1Pkcs12MacData>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12 {
	pub elem : Asn1Seq<Asn1Pkcs12Elem>,
}

#[asn1_obj_selector(selector=val,any=default,x509cert="1.2.840.113549.1.9.22.1")]
#[derive(Clone)]
pub struct Asn1Pkcs12BagsSelector {
	pub val : Asn1Object,
}


#[asn1_choice(selector=valid)]
#[derive(Clone)]
pub struct Asn1Pkcs12BagsElem {
	pub valid : Asn1Pkcs12BagsSelector,
	pub x509cert : Asn1ImpSet<Asn1OctData,0>,
	pub any :Asn1Any,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12Bags {
	pub elem :Asn1Seq<Asn1Pkcs12BagsElem>,
}

#[asn1_obj_selector(selector=val,any=default,shkeybag="1.2.840.113549.1.12.10.1.2",bag=["1.2.840.113549.1.12.10.1.3"])]
#[derive(Clone)]
pub struct Asn1Pkcs12SafeBagSelector {
	pub val : Asn1Object,
}

#[asn1_choice(selector=valid)]
#[derive(Clone)]
pub struct Asn1Pkcs12SafeBagSelectElem {
	pub valid : Asn1Pkcs12SafeBagSelector,
	pub shkeybag : Asn1ImpSet<Asn1X509Sig,0>,
	pub bag : Asn1ImpSet<Asn1Pkcs12Bags,0>,
	pub any :Asn1Any,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12SafeBagElem {
	pub selectelem : Asn1Pkcs12SafeBagSelectElem,
	pub attrib : Asn1Opt<Asn1Set<Asn1X509Attribute>>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1Pkcs12SafeBag {
	pub elem : Asn1Seq<Asn1Pkcs12SafeBagElem>,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPubkeyFormElem {
	pub algor : Asn1X509Algor,
	pub data  : Asn1BitData,
}

#[asn1_sequence()]
#[derive(Clone)]
pub struct Asn1RsaPubkeyForm {
	pub elem :Asn1Seq<Asn1RsaPubkeyFormElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct Asn1OtherNameElem {
	pub typeid :Asn1Object,
	pub value :Asn1Ndef<Asn1Any,0>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct Asn1OtherName {
	pub elem :Asn1Seq<Asn1OtherNameElem>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct Asn1EdiPartyNameElem {
	pub nameassigner :Asn1Opt<Asn1Ndef<Asn1PrintableString,0>>,
	pub partyname :Asn1Ndef<Asn1PrintableString,1>,
}

#[derive(Clone)]
#[asn1_sequence()]
pub struct Asn1EdiPartyName {
	pub elem :Asn1Seq<Asn1EdiPartyNameElem>,
}

#[asn1_int_choice(debug=0,selector=stype,othername=0,rfc822name=1,dnsname=2,directoryname=4,uri=6,ipaddress=7,registerid=8)]
#[derive(Clone)]
pub struct Asn1GeneralName {
	pub stype :i32,
	pub othername : Asn1Imp<Asn1OtherName,0>,
	pub rfc822name :Asn1Imp<Asn1IA5String,1>,
	pub dnsname :Asn1Imp<Asn1IA5String,2>,
	pub directoryname : Asn1Imp<Asn1Seq<Asn1X509Name>,4>,
	pub uri : Asn1Imp<Asn1IA5String,6>,
	pub ipaddress :Asn1Imp<Asn1IA5String,7>,
	pub registerid :Asn1Imp<Asn1Object,8>,
}

pub type HmacSha256 = Hmac<Sha256>;

pub fn calc_hmac_sha256(initkey :&[u8],data :&[u8]) -> Vec<u8> {
	let mut hmac = HmacSha256::new_from_slice(initkey).unwrap();
	hmac.update(data);
	let res = hmac.finalize();
	return res.into_bytes().to_vec();
}


pub fn get_hmac_sha256_key(passv8 :&[u8], saltv8 :&[u8], itertimes : usize) -> Vec<u8> {
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

pub fn get_algor_pbkdf2_private_data(x509algorbytes :&[u8],encdata :&[u8],passin :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
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
            asn1obj_new_error!{Asn1DefError,"not support OID_PBKDF2 types [{}]", types}
        }
        asn1obj_new_error!{Asn1DefError,"not support OID_PBES2 types [{}]",pbe2types}
    }
    asn1obj_new_error!{Asn1DefError,"can not support types [{}]", types}
}

pub fn get_private_key(x509sigbytes :&[u8],passin :&[u8]) -> Result<Asn1RsaPrivateKey,Box<dyn Error>> {
	let mut x509sig = Asn1X509Sig::init_asn1();
	let mut ores = x509sig.decode_asn1(x509sigbytes);
	if ores.is_err() {
		let s :&str = std::str::from_utf8(x509sigbytes)?;
		let (code,_) = pem_to_der(s)?;
		ores = x509sig.decode_asn1(&code);
	}
	if ores.is_err() {
		let e = Err(ores.err().unwrap());
		return e;
	}
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
	asn1obj_new_error!{Asn1DefError,"not support [{}]",types}
}

pub fn get_private_key_file(pemfile :&str,passin :&[u8]) -> Result<Asn1RsaPrivateKey,Box<dyn Error>> {
	let pemdata = read_file(pemfile)?;
	let (derdata,_) = pem_to_der(&pemdata)?;
	return get_private_key(&derdata,passin);
}

pub fn get_rsa_private_key(pemfile :&str, passin :&[u8]) -> Result<RsaPrivateKey, Box<dyn Error>> {
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
