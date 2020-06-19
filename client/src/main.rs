extern crate tungstenite;
extern crate url;
#[macro_use] 
extern crate log;

use tungstenite::{connect, Message};
use url::Url;
use log::Level;
use std::io;
//mod game;
fn main() {
    //game::run();
    env_logger::init();

    let (mut socket, response) = connect(Url::parse("ws://127.0.0.1:3001").unwrap()).expect("aaaa");

    println!("Connected to server");

    loop {
        let mut input = String::new();
        println!("Say something:");
        let mut _new_num = io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line");
          socket.write_message(Message::Text(input.into())).unwrap();
    }
    
}
