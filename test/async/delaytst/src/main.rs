use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
//use std::cell::Box;
use std::error::Error;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn main_func() -> Result<(),Box<dyn Error>> {
    let when = Instant::now() + Duration::from_millis(2000);
    let future = Delay { when };

    let out = future.await;
    println!("out [{}]",out);
    //assert_eq!(out, "done");
    Ok(())
}

fn main() -> Result<(),Box<dyn Error>> {
	let _ =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(main_func())?;
	Ok(())
}