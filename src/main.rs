use mini_redis::{ Connection, Frame};
use tokio::net::{TcpListener,TcpStream};

#[tokio::main]
async fn main() {

    // Connect to the mini-redis Based on any communication design pattern, engineers may use a number of protocols for concrete communication. While core transport vehicles are limited to either TCP or UDP, tons of industry specific protocols are built on top of these two to address certain problems and use cases. Examples of these high level protocols are HTTP/1.1, HTTP/2, HTTP/3, gRPC, WebRTC and many more. Other transport protocols like QUIC was built on top of UDP to bring HTTP/2 streaming down at the transport level. Each protocol has its pros and cons and fits certain use cases. In the course, I discuss the top common protocols and provide examples and demos where applicable.



    // let mut client = client::connect("127.0.0.1:6379").await?;

    // Set a key-value pair
    // client.set("hello", "world".into()).await?;

    // Get the value using the key
    // let result = client.get("hello").await?;

    // Print the result stored in mini redis
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
