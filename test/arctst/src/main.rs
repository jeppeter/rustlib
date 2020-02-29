use std::sync::Arc;

fn  main()  {
	let foo = Arc::new(vec![3.2,63.2,500.2]);
	//let mut i :i32;
	//i  = 0;
	for i  in  0..3 {
		println!("[{}]=[{}]",i,foo[i]);
		//i +=  1;
	}
	return;
}