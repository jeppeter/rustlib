
mod impls;

use impls::*;
use std::error::Error;
use std::cell::RefCell;
use std::sync::Arc;

struct Sha256DigestAlgo {

}

impl Sha256DigestAlgo {
	fn new() -> Self {
		Self {}
	}
}

impl Asn1DigestOp for Sha256DigestAlgo {
	fn digest_update(&mut self, _data :&[u8]) -> Result<(),Box<dyn Error>> {
		println!("Sha256DigestAlgo digest_update");
		Ok(())
	}

	fn digest_final(&mut self) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("Sha256DigestAlgo digest_final");
		Ok(vec![])
	}
}

struct RsaSignVfy {

}

impl RsaSignVfy {
	fn new() -> Self {
		Self {}
	}
}

impl Asn1SignOp for RsaSignVfy {
	fn sign_update(&mut self,data :&[u8],digop :Arc<RefCell<dyn Asn1DigestOp>>) -> Result<(),Box<dyn Error>> {
		println!("RsaSignVfy sign_update");
		return digop.borrow_mut().digest_update(data);
	}
	fn sign_final(&mut self,digop:Arc<RefCell<dyn Asn1DigestOp>>) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("RsaSignVfy sign_final");
		return digop.borrow_mut().digest_final();
	}
}

impl Asn1VerifyOp for RsaSignVfy {
	fn verify_update(&mut self, origdata :&[u8], digop :Arc<RefCell<dyn Asn1DigestOp>>) -> Result<(),Box<dyn Error>> {
		println!("RsaSignVfy sign_update");
		let _ = digop.borrow_mut().digest_update(origdata)?;
		Ok(())
	}
	fn verify_final(&mut self,signdata :&[u8], digop :Arc<RefCell<dyn Asn1DigestOp>>) -> Result<bool,Box<dyn Error>> {
		println!("RsaSignVfy sign_final");
		let digdata = digop.borrow_mut().digest_final()?;
		let mut retv :bool = false;
		if digdata == signdata {
			retv = true;
		}
		Ok(retv)
	}
}


struct AesEncDe {

}

impl AesEncDe {
	fn new() -> Self {
		Self {}
	}
}

impl Asn1EncryptOp for AesEncDe {
	fn encrypt_update(&mut self, _data :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("AesEncDe encrypt_update");
		Ok(vec![])
	}
	fn encrypt_final(&mut self) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("AesEncDe encrypt_final");
		Ok(vec![])
	}
}


impl Asn1DecryptOp for AesEncDe {
	fn decrypt_update(&mut self, _data :&[u8]) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("AesEncDe decrypt_update");
		Ok(vec![])
	}
	fn decrypt_final(&mut self) -> Result<Vec<u8>,Box<dyn Error>> {
		println!("AesEncDe decrypt_final");
		Ok(vec![])
	}
}


fn main() -> Result<(),Box<dyn Error>> {
    let s = Sha256DigestAlgo::new();
    let cdata :Vec<u8> = vec![3];
    let cc :Arc<RefCell<Sha256DigestAlgo>> = Arc::new(RefCell::new(s));
    let mut rsav = RsaSignVfy::new();
    let _ = rsav.sign_update(&cdata,cc.clone())?;
    let cmpdata = rsav.sign_final(cc.clone())?;
    println!("data {:?}", cmpdata);
    let _ = rsav.verify_update(&cdata,cc.clone())?;
    let _ = rsav.verify_final(&cdata,cc.clone())?;

    let mut aesv = AesEncDe::new();
    let _ = aesv.encrypt_update(&cdata)?;
    let _ = aesv.encrypt_final()?;

    let _ = aesv.decrypt_update(&cdata)?;
    let _ = aesv.decrypt_final()?;
    Ok(())
}
