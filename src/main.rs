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
    use rand::Rng;
    let duration_sec = rand::thread_rng().gen_range(0, 10);
    tokio::time::delay_for(Duration::from_secs(duration_sec)).await;
    println!("Hello, world!");
}
