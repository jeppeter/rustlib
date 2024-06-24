
use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::env;
use std::error::Error;
use rand::Rng;

pub fn parse_u64(instr :&str) -> Result<u64,Box<dyn Error>> {
    let mut cparse = format!("{}",instr);
    let mut base :u32 = 10;
    let retv :u64;
    if cparse.starts_with("0x") || cparse.starts_with("0X") {
        cparse = cparse[2..].to_string();
        base = 16;
    } else if cparse.starts_with("x") || cparse.starts_with("X") {
        cparse = cparse[1..].to_string();
        base = 16;
    }

    match u64::from_str_radix(&cparse,base) {
        Ok(v) => {
            retv = v;
        },
        Err(e) => {
            return Err(Box::new(e));
        }
    }
    Ok(retv)
}

async fn connect_val(addr :String,v :u64) {
   let mut stream = TcpStream::connect(&addr).await.expect("no connectr");
   //let mut rng = rand::thread_rng();
   let mut cnt : i32 = 0;
   let laddr = stream.local_addr().unwrap();
   println!("client {}",laddr);
   loop {
    let bval :u64= {
        let mut rng = rand::thread_rng();
        rng.gen::<u64>() & 0xffff
    };
    let s = format!("{} bval {}",v,bval);

    stream.write_all(s.as_bytes()).await.expect("write error");
    let mut buf = vec![0; 1024];

    let n = stream.read(&mut buf).await.expect("read error");
    if n == 0 {
        return;
    }
    let rs = String::from_utf8_lossy(&buf[0..n]);
    cnt += 1;
    if (cnt % 100) == 1 {
        println!("{}:write [{}]",laddr,s);
        println!("{}:read {}",laddr,rs);        
    }


    tokio::time::sleep(tokio::time::Duration::from_millis(bval & 0xfff)).await;
}
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let numstr = env::args().nth(2).unwrap_or_else(|| "1".to_string());
    let num :u64 = parse_u64(&numstr)?;

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.

    for v in 0..num {
        let nstr = format!("{}",addr);
        tokio::spawn(connect_val(nstr,v));
    }

    tokio::select!{
        _ = tokio::signal::ctrl_c()  => {
            println!("ctrlc ");
        }
    }
    Ok(())
}
