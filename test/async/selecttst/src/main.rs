use std::error::Error;
use tokio;

async fn do_stuff_async() -> Result<i32,Box<dyn Error>> {
    println!("do_stuff_async");
    Ok(1)
}

async fn more_async_work() -> Result<i32,Box<dyn Error>> {
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