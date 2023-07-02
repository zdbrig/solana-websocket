use {
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result as PluginResult
    },
    solana_program::pubkey::Pubkey,
    std::error::Error,
    websocket::client::ClientBuilder,
    websocket::{OwnedMessage},
    std::sync::{Arc, Mutex},
    std::fmt,
};

pub struct SimplePlugin {
    client: Option<Arc<Mutex<websocket::sync::Client<std::net::TcpStream>>>>,
}

impl fmt::Debug for SimplePlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimplePlugin")
            .field("client", &self.client.is_some())
            .finish()
    }
}

impl Default for SimplePlugin {
    fn default() -> Self {
        SimplePlugin {
            client: None,
        }
    }
}

impl GeyserPlugin for SimplePlugin {

    fn name(&self) -> &'static str {
        "simple-geyser"
    }

    fn on_load(&mut self, _config_file: &str) -> PluginResult<()> {
        let client = ClientBuilder::new("ws://127.0.0.1:2794")
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()
            .unwrap();

        self.client = Some(Arc::new(Mutex::new(client)));
        Ok(())
    }

    fn on_unload(&mut self) {
        if let Some(client) = &self.client {
            let mut locked_client = client.lock().unwrap();
            let message = OwnedMessage::Close(None);
            let _ = locked_client.send_message(&message);
        }
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from("Some error message for V0_0_1")));
            }
            ReplicaAccountInfoVersions::V0_0_2(_) => {
                return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from("Some error message for V0_0_2")));
            }
            ReplicaAccountInfoVersions::V0_0_3(account_info) => {
                account_info
            }
        };

        let pk = Pubkey::new(account_info.pubkey);
        let message = format!("account {:#?} updated at slot {}!", pk, slot);

        if let Some(client) = &self.client {
            let mut locked_client = client.lock().unwrap();
            let message = OwnedMessage::Text(message);
            match locked_client.send_message(&message) {
                Ok(_) => (),
                Err(_) => return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from("Failed to send message"))),
            }
        } else {
            return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from("No client available")));
        }

        if is_startup {
            Ok(())
        } else {
            Ok(())
        }
    }

    
}
