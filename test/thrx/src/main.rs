// valid
use std::{thread,time};

fn main() {
  let v = vec![1,2,3];
  let  mut thrs  = Vec::new();
  for _ in 1..5 {
	  let mut vcol = v.clone();
	  let  t  =  thread::spawn(move  || {
	     vcol.push(4);
	     thread::sleep(time::Duration::from_millis(3000));
	     println!("v {:?}", vcol);
	  });
	  thrs.push(t);
  }
  // Can no longer access `v` here.
  println!("main {:?}", v);
  for t in thrs.iter() {
  	t.join();
  }

}