use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    let task = task::spawn(hello());
    println!("Going to hello");
    match task.await {
        Ok(_) => {
            println!("You've been helloed");
        }
        Err(_) => println!("Hello failed"),
    }
}

async fn hello() {
    tokio::time::delay_for(Duration::from_secs(6)).await;
    println!("Hello, world!");
}
