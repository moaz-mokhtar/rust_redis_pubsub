mod message;
mod message_handler;
mod redis_publisher;
mod redis_subscriber;

extern crate redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("client started");

    let client = redis::Client::open("redis://localhost/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("order")?;

    loop {
        let msg = pubsub.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }

    // if let Err(error) = redis_subscriber::subscribe(String::from("order")) {
    //     println!("{:?}", error);
    //     panic!("{:?}", error);
    // } else {
    //     println!("connected to queue");
    // }



    println!("client finished");

    Ok(())
}
