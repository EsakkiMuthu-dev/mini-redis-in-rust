use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the mini-redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set a key-value pair
    client.set("hello", "world".into()).await?;

    // Get the value using the key
    let result = client.get("hello").await?;

    // Print the result stored in mini redis
    println!("<------------>");
    println!();
    println!("The result stored in mini redis is {:?}", result);
    println!();
    println!("<------------>");
    Ok(())
}
