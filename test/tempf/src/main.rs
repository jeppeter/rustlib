use std::io::{Write,Read,Seek,SeekFrom};
use tempfile::{NamedTempFile};
use std::thread;
use std::time;
//use tempfile::tempdir;


fn main() {
    let _f :NamedTempFile = NamedTempFile::new().unwrap();
    let mut f1 = _f.reopen().unwrap();
    f1.write_all(b"hello from").unwrap();
    f1.sync_all().unwrap();
    //f1.close();
    println!("path {}", _f.path().display());

    loop {
    	thread::sleep(time::Duration::from_millis(10000));
    	println!("sleep");
    }
}
