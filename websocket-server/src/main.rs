extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
	let server = Server::bind("127.0.0.1:2794").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let (mut receiver, _) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = match message {
					Ok(msg) => msg,
					Err(e) => {
						println!("Error: {}", e);
						return;
					}
				};

				match message {
					OwnedMessage::Close(_) => {
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						println!("Ping message: {:?}", ping);
					}
					OwnedMessage::Text(txt) => {
						println!("Received Text: {}", txt);
					}
					OwnedMessage::Binary(bin) => {
						println!("Received Binary Data: {:?}", bin);
					}
					_ => (),
				}
			}
		});
	}
}
