use std::sync::RwLock;


#[tokio::main]
pub async fn main() {
    let value = RwLock::new(22);
    let result2 = *value.read().unwrap();
    println!("{result2:?}"); // prints 88
    moro::async_scope!(|scope| {
        scope.spawn(async {
	        // we can spawn nested tasks
            scope.spawn(async {
	            // and access values that outlive the scope
                *value.write().unwrap() *= 3; 
            });

            *value.write().unwrap() += 3;
        });
    })
    .await;
    let result = *value.read().unwrap();
    println!("{result:?}"); // prints 69 for it will give 
}