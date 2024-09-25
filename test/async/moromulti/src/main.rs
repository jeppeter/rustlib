use std::sync::RwLock;


#[tokio::main]
pub async fn main() {
    let value = RwLock::new(22);
    let result2 = *value.read().unwrap();
    println!("threadid {:?} {result2:?}",  std::thread::current().id());
    moro::async_scope!(|scope| {
        scope.spawn(async {
	        // we can spawn nested tasks
            scope.spawn(async {
	            // and access values that outlive the scope
                *value.write().unwrap() *= 3; 
                println!("threadid {:?} *3",  std::thread::current().id());
            });

            *value.write().unwrap() += 3;
            println!("threadid {:?} +3",  std::thread::current().id());
        });
    })
    .await;
    let result = *value.read().unwrap();
    println!("threadid {:?} {result:?}",  std::thread::current().id());
}