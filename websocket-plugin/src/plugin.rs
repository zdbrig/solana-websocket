use super::simple_plugin::SimplePlugin;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin,
    GeyserPluginError,
    ReplicaAccountInfoVersions,
    Result as PluginResult,
};
use solana_program::pubkey::Pubkey;
use std::error::Error;
use serde_json::json;
use reqwest::StatusCode;

impl GeyserPlugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "simple-geyser"
    }


    fn on_load(&mut self, _config_file: &str) -> PluginResult<()> {
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        _is_startup: bool
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
            ReplicaAccountInfoVersions::V0_0_3(account_info) => account_info,
        };

        let pubkeys_guard = self.pubkeys.lock().unwrap();
        
        for pubkey_option in pubkeys_guard.iter() {
            if let Some(pubkey) = pubkey_option {
                let account_pubkey = account_info.pubkey;
                let pubkey_bytes = pubkey.to_bytes();

                if &account_pubkey[..] != &pubkey_bytes[..] {
                    continue;
                }

                let pk = Pubkey::new(account_info.pubkey);

                let payload = json!({
                    "pubkey": account_info.pubkey,
                    "slot": slot,
                });

                let handle = std::thread::spawn(move || {
                    let response = reqwest::blocking::Client::new()
                        .post("http://localhost:3000/account")
                        .json(&payload)
                        .send();

                    match response {
                        Ok(response) => {
                            if response.status() == StatusCode::OK {
                                println!("Request successful!");
                            } else {
                                println!("Request failed with status code: {}", response.status());
                            }
                        }
                        Err(err) => {
                            println!("An error occurred during the request: {}", err);
                        }
                    }
                });

                // Wait for the thread to complete
                handle.join().unwrap();
            }
        }
        Ok(())
    }

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}
