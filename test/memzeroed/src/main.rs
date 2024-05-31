use std::mem::MaybeUninit;

#[allow(dead_code)]
#[derive(Debug)]
struct OneZero {
	ccv : i32,
	ddv : i64,
	zzf : f32,
	bbf : f64,
}


fn main() {
    let mut retv :OneZero = OneZero {
    	ccv : 3 ,
    	.. unsafe {
    		MaybeUninit::<OneZero>::zeroed().assume_init()
    	}
    };

    println!("retv {:?}", retv);

    retv.zzf = 7.0;
	println!("retv {:?}", retv);
	return;    
}
