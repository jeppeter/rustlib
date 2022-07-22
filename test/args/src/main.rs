use std::env;
use std::i64;
use std::io::{Write,BufWriter};
use std::borrow::Cow;

#[derive(Clone)]
pub struct Asn1Integer {
	pub ival :i64,
}

impl Asn1Integer {
	pub fn init_asn1() ->Self {
		Asn1Integer{
			ival : 0,
		}
	}
}

#[derive(Clone)]
pub struct Asn1Ndef<T: Clone,const TAG :u8=0> {
	pub data :Vec<T>,
	tag :u8,
}

impl<T: Clone,const TAG :u8> Asn1Ndef<T,TAG> {
	pub fn init_asn1() -> Self {
		Asn1Ndef {
			data :Vec::new(),
			tag :TAG,
		}
	}
}

#[derive(Clone)]
pub struct Asn1Obj {
	pub mname :Asn1Ndef<Asn1Integer,5>,
}

impl Asn1Obj {
	pub fn init_asn1() ->Self {
		Asn1Obj {
			mname : Asn1Ndef::init_asn1(),
		}
	}
}

pub fn basename<'a>(path: &'a str) -> Cow<'a, str> {
	let splc :char;
	if env::consts::OS == "windows" {
		splc = '\\';
	} else {
		splc = '/';
	}
    let mut pieces = path.rsplitn(2, |c| c == splc);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

fn write_fmts<T : Write>(iowrite :&mut T,s :&str) -> Result<usize,std::io::Error> {
	let mut c = iowrite.write(s.as_bytes())?;
	c += iowrite.write(b"\n")?;
	Ok(c)
	//iowrite.write_str("\n")
}

fn main() {
	let mut i=0;
	let mut v :i64;
	let mut base :u32;
	let mut cparse :String;
	let b = format!(r#"get {{ {} }}"#,"ccs");
	let mut av :Asn1Obj = Asn1Obj::init_asn1();
	let mut va :Asn1Ndef<Asn1Integer> = Asn1Ndef::init_asn1();
	

	//let mut wstr :String = String::new();
	let mut buf = vec![];
	{
		let mut wstr = BufWriter::new(&mut buf);
		//let mut wstr = std::io::stdout();

		let mut cstr :String;
		cstr = format!("OS={}",env::consts::OS);
		write_fmts(&mut wstr,&cstr).unwrap();
		write_fmts(&mut wstr,&b).unwrap();
		write_fmts(&mut wstr,&format!("tag {} va {}", av.mname.tag,va.tag)).unwrap();

	    for arg in env::args() {
	    	if i == 0 {
	    		cstr = format!("prog={}",basename(&arg));
	    		write_fmts(&mut wstr,&cstr).unwrap();
	    	}
	    	cstr = format!("[{}]=[{}]", i, arg);
	    	write_fmts(&mut wstr,&cstr).unwrap();	    	
	    	//writeln!(wstr,"[{}]=[{}]", i, arg).unwrap();
	    	i += 1;
	    	if i == 1 {
	    		continue;
	    	}
	    	base = 10;
	    	cparse = format!("{}",arg);
	    	if arg.starts_with("0x") || arg.starts_with("0X") {
	    		cparse = cparse[2..].to_string();
	    		base = 16;
	    	}
	    	match i64::from_str_radix(&cparse,base) {
	    		Ok(c) => {
	    			v=  c;
			    	//println!("[{}]={}",arg,v);
			    	cstr = format!("[{}]={}",arg,v);
			    	write_fmts(&mut wstr, &cstr).unwrap();
			    	//writeln!(wstr,"[{}]={}",arg,v).unwrap();

	    		},
	    		Err(e) => {
	    			//println!("[{}] error[{:?}]", arg,e);
	    			cstr = format!("[{}] error[{:?}]", arg,e);
	    			write_fmts(&mut wstr,&cstr).unwrap();
	    		}
	    	}
	    }
	}
    let s = std::str::from_utf8(&buf).unwrap();
    print!("total str\n{}",s);
}
