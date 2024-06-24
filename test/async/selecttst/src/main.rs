use std::error::Error;
use tokio;
use rand::Rng;

async fn do_stuff_async() -> Result<i32,Box<dyn Error>> {
	let mut rng = rand::thread_rng();
	loop {
		let bval :u64 = rng.gen::<u64>() & 0xffff;
		if bval < 100  ||  bval > ((1 << 16) - 100) {
			println!("do_stuff_async bval {}", bval);
			break;
		}
		println!("do_stuff_async wait {}", bval & 0xfff);
		tokio::time::sleep(tokio::time::Duration::from_millis(bval & 0xfff)).await;
	}
    println!("do_stuff_async");
    Ok(1)
}

async fn more_async_work() -> Result<i32,Box<dyn Error>> {
	let mut rng = rand::thread_rng();
	loop {
		let bval :u64 = rng.gen::<u64>() & 0xffff;
		if bval < 100  ||  bval > ((1 << 16) - 100) {
			println!("more_async_work bval {}", bval);
			break;
		}
		println!("more_async_work wait {}", bval & 0xfff);
		tokio::time::sleep(tokio::time::Duration::from_millis(bval & 0xfff)).await;
	}
    println!("more_async_work");
    Ok(2)
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = more_async_work() => {
            println!("more_async_work() completed first")
        }
    };
}