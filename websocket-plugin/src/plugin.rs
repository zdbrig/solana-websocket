use ::{
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin,
        GeyserPluginError,
        ReplicaAccountInfoVersions,
        Result as PluginResult,
    },
    solana_program::pubkey::Pubkey,
    solana_program::pubkey::ParsePubkeyError,
    std::error::Error,
    tungstenite::{ accept, Message },
    std::sync::{ Arc, Mutex },
    std::net::{ TcpListener },
    std::fmt,
    std::str::FromStr,
    std::thread,
};

pub struct SimplePlugin {
    server: Arc<Mutex<Option<tungstenite::WebSocket<std::net::TcpStream>>>>,
    pubkey: Arc<Mutex<Option<Pubkey>>>,
}

impl fmt::Debug for SimplePlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimplePlugin")
            .field("server", &self.server.lock().unwrap().is_some())
            .finish()
    }
}

impl Default for SimplePlugin {
    fn default() -> Self {
        SimplePlugin {
            server: Arc::new(Mutex::new(None)),
            pubkey: Arc::new(Mutex::new(None)),
        }
    }
}

impl GeyserPlugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "simple-geyser"
    }

    fn on_load(&mut self, _config_file: &str) -> PluginResult<()> {
        let server = Arc::clone(&self.server);
        let pubkey = Arc::clone(&self.pubkey);
        thread::spawn(move || {
            let tcp_server = TcpListener::bind("127.0.0.1:2794").unwrap();
            for stream in tcp_server.incoming() {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = websocket.read_message().unwrap();
                    if msg.is_text() {
                        let text = msg.into_text().unwrap();
                        if text.starts_with("start ") {
                            let pubkey_string = text[6..].to_string();
                            let pk = Pubkey::from_str(&pubkey_string).unwrap();
                            println!("Received public key: {}", pk); // Print the public key received from the client
                            *pubkey.lock().unwrap() = Some(pk);
                            break;
                        }
                    }
                }
                *server.lock().unwrap() = Some(websocket);
                break;
            }
        });

        Ok(())
    }

    fn on_unload(&mut self) {
        // Here, you may want to close your WebSocket connection
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                return Err(
                    GeyserPluginError::Custom(
                        Box::<dyn Error + Send + Sync>::from("Some error message for V0_0_1")
                    )
                );
            }
            ReplicaAccountInfoVersions::V0_0_2(_) => {
                return Err(
                    GeyserPluginError::Custom(
                        Box::<dyn Error + Send + Sync>::from("Some error message for V0_0_2")
                    )
                );
            }
            ReplicaAccountInfoVersions::V0_0_3(account_info) => { account_info }
        };

        let pk = Pubkey::new(account_info.pubkey);
        println!("Updated public key: {}", pk); // Print the updated account's public key

        let mut locked_server = self.server.lock().unwrap();
        if let Some(server) = &mut *locked_server {
            let mut locked_pubkey = self.pubkey.lock().unwrap();
            if let Some(pubkey) = &*locked_pubkey {
                if &pk == pubkey {
                    let message = format!("Account {:#?} updated at slot {}!", pk, slot);
                    match server.write_message(Message::Text(message)) {
                        Ok(_) => (),
                        Err(_) => {
                            return Err(
                                GeyserPluginError::Custom(
                                    Box::<dyn Error + Send + Sync>::from("Failed to send message")
                                )
                            );
                        }
                    }
                }
            } else {
                return Err(
                    GeyserPluginError::Custom(
                        Box::<dyn Error + Send + Sync>::from("No pubkey available")
                    )
                );
            }
        } else {
            return Err(
                GeyserPluginError::Custom(
                    Box::<dyn Error + Send + Sync>::from("No server available")
                )
            );
        }

        if is_startup {
            Ok(())
        } else {
            Ok(())
        }
    }
}
