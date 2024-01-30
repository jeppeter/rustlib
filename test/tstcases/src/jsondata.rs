
use std::sync::{Arc,RwLock};
use serde_json::{Value,json};
use std::collections::HashMap;
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use std::error::Error;

extargs_error_class!{JsonError}

struct JSonPackInner {
	vals :HashMap<String,Value>,
}

impl Drop for JSonPackInner {
	fn drop(&mut self) {
		self.close();
	}
}

impl JSonPackInner {
	fn new(s :&str) -> Result<Arc<RwLock<Self>>,Box<dyn Error>> {
		let v :HashMap<String,Value> = serde_json::from_str(s)?;
		let retv :Self = Self {
			vals :v,
		};
		Ok(Arc::new(RwLock::new(retv)))
	}

	fn close(&mut self) {
		self.vals = HashMap::new();
		return;
	}

	fn unpack(b :&[u8]) -> Result<Arc<RwLock<Self>>,Box<dyn Error>> {
		if b.len() < 8 {
			extargs_new_error!{JsonError,"len({}) < 8",b.len()}
		}
		if (b[0] != 'J' as u8) || 
			(b[1] != 'S' as u8) || 
			(b[2] != 'O' as u8) ||
			(b[3] != 'N' as u8) {
				extargs_new_error!{JsonError,"b[0..4] 0x{:x}0x{:x}0x{:x}0x{:x} ",b[0],b[1],b[2],b[3]}
			}
		let mut clen :usize = 0;
		let mut idx :usize = 0;
		while idx  < 4 {
			clen |= (b[idx + 4] as usize) << (8 * (3-idx));
			idx += 1;
		}
		if clen < b.len() {
			extargs_new_error!{JsonError,"clen 0x{:x} < b.len 0x{:x} ",clen,b.len()}
		}
		let s :String = String::from_utf8_lossy(&b[8..clen]).to_string();
		return Self::new(&s);
	}

	fn pack(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let s = json!(self.vals).to_string();
		let mut retv = s.as_bytes().to_vec();
		/*for last one*/
		//retv.push(0);
		let clen :usize = retv.len() + 8;
		let mut idx :usize = 0;
		while idx < 4 {
			let cb :u8 = ((clen >> (8 *(idx)) ) as u8) & 0xff;
			retv.insert(0,cb);
			idx += 1;
		}

		retv.insert(0,'N' as u8);
		retv.insert(0,'O' as u8);
		retv.insert(0,'S' as u8);
		retv.insert(0,'J' as u8);
		Ok(retv)
	}

	fn merge_unpack_ref(&mut self,other :&JSonUnpackInner,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}

	fn merge_pack_ref(&mut self,other :&JSonPackInner,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}


	fn merge_unpack(&mut self,other :&JSonUnpackInner,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}

	fn merge_pack(&mut self,other :&JSonPackInner,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}

}

struct JSonUnpackInner {
	vals :HashMap<String,Value>,
}


impl Drop for JSonUnpackInner {
	fn drop(&mut self) {
		self.close();
	}
}

impl JSonUnpackInner {
	fn new(s :&str) -> Result<Arc<RwLock<Self>>,Box<dyn Error>> {
		let v :HashMap<String,Value> = serde_json::from_str(s)?;
		let retv :Self = Self {
			vals :v,
		};
		Ok(Arc::new(RwLock::new(retv)))
	}

	fn close(&mut self) {
		self.vals = HashMap::new();
		return;
	}
	fn unpack(b :&[u8]) -> Result<Arc<RwLock<Self>>,Box<dyn Error>> {
		if b.len() < 8 {
			extargs_new_error!{JsonError,"len({}) < 8",b.len()}
		}
		if (b[0] != 'J' as u8) || 
			(b[1] != 'S' as u8) || 
			(b[2] != 'O' as u8) ||
			(b[3] != 'N' as u8) {
				extargs_new_error!{JsonError,"b[0..4] 0x{:x}0x{:x}0x{:x}0x{:x} ",b[0],b[1],b[2],b[3]}
			}
		let mut clen :usize = 0;
		let mut idx :usize = 0;
		while idx  < 4 {
			clen |= (b[idx + 4] as usize) << (8 * (3-idx));
			idx += 1;
		}
		if clen < b.len() {
			extargs_new_error!{JsonError,"clen 0x{:x} < b.len 0x{:x} ",clen,b.len()}
		}
		let s :String = String::from_utf8_lossy(&b[8..clen]).to_string();
		return Self::new(&s);
	}

	fn pack(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let s = json!(self.vals).to_string();
		let mut retv = s.as_bytes().to_vec();
		//retv.push(0);
		let clen :usize = retv.len() + 8;
		let mut idx :usize = 0;
		while idx < 4 {
			let cb :u8 = ((clen >> (8 *(idx)) ) as u8) & 0xff;
			retv.insert(0,cb);
			idx += 1;
		}

		retv.insert(0,'N' as u8);
		retv.insert(0,'O' as u8);
		retv.insert(0,'S' as u8);
		retv.insert(0,'J' as u8);
		Ok(retv)
	}

	fn merge_unpack_ref(&mut self,other :&JSonUnpackInner,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}

	fn merge_pack_ref(&mut self,other :&JSonPackInner,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}


	fn merge_unpack(&mut self,other :&JSonUnpackInner,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}

	fn merge_pack(&mut self,other :&JSonPackInner,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		for k in keywords.iter() {
			let nk = format!("{}",k);
			match other.vals.get(&nk) {
				Some(v) => {
					self.vals.insert(nk,v.clone());
				},
				_ => {}
			}
		}
		Ok(())
	}
}


pub struct JSonPack {
	inner :Arc<RwLock<JSonPackInner>>,
}

impl Drop for JSonPack {
	fn drop(&mut self) {
		self.close();
	}
}

pub struct JSonUnpack {
	inner :Arc<RwLock<JSonUnpackInner>>,
}

impl Drop for JSonUnpack {
	fn drop(&mut self) {
		self.close();
	}
}

#[allow(dead_code)]
impl JSonPack {
	pub fn new(s :&str) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner : JSonPackInner::new(s)?,
		};
		Ok(retv)
	}

	pub fn close(&mut self) {
		return;
	}

	pub fn unpack(b :&[u8]) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner : JSonPackInner::unpack(b)?,
		};
		Ok(retv)
	}

	pub fn pack(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let bres = self.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()};
		}
		let b = bres.unwrap();
		let retv = b.pack();
		return retv;
	}

	pub fn merge_unpack_ref(&mut self,other :&JSonUnpack,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_unpack_ref(&o,keywords);
	}

	pub fn merge_pack_ref(&mut self,other :&JSonPack,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_pack_ref(&o,keywords);
	}


	pub fn merge_unpack(&mut self,other :&JSonUnpack,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_unpack(&o,keywords);
	}

	pub fn merge_pack(&mut self,other :&JSonPack,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_pack(&o,keywords);
	}

}

#[allow(dead_code)]
impl JSonUnpack {
	pub fn new(s :&str) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner : JSonUnpackInner::new(s)?,
		};
		Ok(retv)
	}

	pub fn close(&mut self) {
		return;
	}

	pub fn unpack(b :&[u8]) -> Result<Self,Box<dyn Error>> {
		let retv :Self = Self {
			inner : JSonUnpackInner::unpack(b)?,
		};
		Ok(retv)
	}

	pub fn pack(&self) -> Result<Vec<u8>,Box<dyn Error>> {
		let bres = self.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()};
		}
		let b = bres.unwrap();
		let retv = b.pack();
		return retv;
	}

	pub fn merge_unpack_ref(&mut self,other :&JSonUnpack,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_unpack_ref(&o,keywords);
	}

	pub fn merge_pack_ref(&mut self,other :&JSonPack,keywords :&[&str]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_pack_ref(&o,keywords);
	}


	pub fn merge_unpack(&mut self,other :&JSonUnpack,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_unpack(&o,keywords);
	}

	pub fn merge_pack(&mut self,other :&JSonPack,keywords :&[String]) -> Result<(),Box<dyn Error>> {
		let bres = self.inner.write();
		let ores = other.inner.read();
		if bres.is_err() {
			extargs_new_error!{JsonError,"{}",bres.err().unwrap()}
		}

		if ores.is_err() {
			extargs_new_error!{JsonError,"{}",ores.err().unwrap()}
		}

		let mut s = bres.unwrap();
		let o = ores.unwrap();
		return s.merge_pack(&o,keywords);
	}

}

