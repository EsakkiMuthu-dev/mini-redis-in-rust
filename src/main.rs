use mini_redis::{ Connection, Frame};
use tokio::net::{TcpListener,TcpStream};

#[tokio::main]
async fn main() {
    // // Connect to the mini-redis server
    // let mut client = client::connect("127.0.0.1:6379").await?;

    // // Set a key-value pair
    // client.set("hello", "world".into()).await?;

    // // Get the value using the key
    // let result = client.get("hello").await?;

    // // Print the result stored in mini redis
    // println!("<------------>");
    // println!();
    // println!("The result stored in mini redis is {:?}", result);
    // println!();
    // println!("<------------>");
    // Ok(())
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
          // The second item contains the IP and port of the new connection.
     let (socket,_port) = listener.accept().await.unwrap();
     process(socket).await;
    }
}

async fn process(socket:TcpStream)
{
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap()
    {
        println!("Starts reading frames");
         println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
