mod exchange;
mod data;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};


struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
    
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "$mexc" {
            let to_send = get_to_send("mex");
            if let Err(why) = msg.channel_id.say(&ctx.http, to_send) {
                println!("Error sending message: {:?}", why);
            }
        }
        
            if msg.content == "$okc" {
            let to_send = get_to_send("ok");
            if let Err(why) = msg.channel_id.say(&ctx.http, to_send) {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn get_to_send(id: &str) -> String {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    match id {
        "mex" => rt.block_on(data::get_mex_cycle()),
        "ok" => rt.block_on(data::get_ok_cycle()),
        _ => String::from("impossible id string"),
    }
}

pub fn launch() {
    let token = "NzIzNTU4MzY1MjYzMDM2NDI4.XxI5Tw.TUpfWXbaHm_sUYtavc9O6m1Vx84";
    let mut client = Client::new(&token, Handler).expect("Error creating the client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}