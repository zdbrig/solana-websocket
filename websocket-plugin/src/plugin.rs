use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin,
    GeyserPluginError,
    ReplicaAccountInfoVersions,
    Result as PluginResult,
};
use solana_program::pubkey::Pubkey;
use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;
use std::str::FromStr;
use reqwest::Client;
use std::thread;
use std::time::Duration;
use serde_json::json;
use reqwest::StatusCode;

#[derive(Debug)]
pub struct SimplePlugin {
    pubkeys: Arc<Mutex<Vec<Option<Pubkey>>>>,
}

impl Default for SimplePlugin {
    fn default() -> Self {
        let pubkeys = Arc::new(Mutex::new(Vec::new()));
        let cloned_pubkeys = Arc::clone(&pubkeys);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                SimplePlugin::start_refresher(cloned_pubkeys).await;
            });
        });

        SimplePlugin { pubkeys }
    }
}

impl SimplePlugin {
    async fn start_refresher(pubkeys: Arc<Mutex<Vec<Option<Pubkey>>>>) {
        let client = Client::new();

        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;

            let response = client.get("http://localhost:3000/pubkey").send().await;

            if let Ok(response) = response {
                if response.status().is_success() {
                    if let Ok(pubkeys_str) = response.text().await {
                        let pubkeys_str = pubkeys_str.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();

                        let mut pubkeys_guard = pubkeys.lock().unwrap();
                        pubkeys_guard.clear();
                        for pubkey_str in pubkeys_str {
                            println!("new pubkey {}", pubkey_str);
                            if let Ok(new_pubkey) = Pubkey::from_str(&pubkey_str) {
                                pubkeys_guard.push(Some(new_pubkey));
                                println!("Refreshed pubkey to: {}", new_pubkey);
                            }
                        }
                    }
                } else {
                    println!("Failed to fetch new pubkeys: HTTP request was not successful");
                }
            } else {
                println!("Failed to fetch new pubkeys: Error making HTTP request");
            }
        }
    }
}

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

                let handle = thread::spawn(move || {
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
