extern crate tungstenite;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
fn main() {

    let server = TcpListener::bind("127.0.0.1:3001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();


                if msg.is_binary() || msg.is_text() {
                    println!("{}", msg);
                }
            }
        });
    }
}
