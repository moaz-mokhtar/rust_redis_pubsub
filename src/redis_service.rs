extern crate redis;

use std::error::Error;
use redis::{Commands, ControlFlow, PubSubCommands};
use crate::message::Message;
use crate::message_handler;

pub async fn start_subscription(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = subscribe(channel).await;
    Ok(())
}

async fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost").unwrap();
        
        let mut con = client.get_connection().unwrap();

        let _: () = con.subscribe(&[channel], |msg| {            
            let received: String = msg.get_payload().unwrap();
            let message_obj = serde_json::from_str::<Message>(&received).unwrap();
            
            message_handler::handle(message_obj);
            return ControlFlow::Continue;
        }).unwrap();
    });

    Ok(())
}

pub fn push_message(message: Message) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost/")?;
    let mut con = client.get_connection().unwrap();

    let json = serde_json::to_string(&message)?;

    let _: () = con.publish(message.channel, json).unwrap();

    Ok(())
}