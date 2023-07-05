use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, Result as PluginResult,
};
use solana_program::pubkey::Pubkey;
use std::error::Error;
use std::str::FromStr;
use reqwest::blocking::Client;

#[derive(Debug)]
pub struct SimplePlugin {
    pubkey: Option<Pubkey>,
}

impl Default for SimplePlugin {
    fn default() -> Self {
        SimplePlugin { pubkey: None }
    }
}

impl GeyserPlugin for SimplePlugin {
    fn name(&self) -> &'static str {
        "simple-geyser"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        // Retrieve the pubkey from http://localhost:3000/pubkey
        let client = Client::new();
        let response = client.get("http://localhost:3000/pubkey").send().unwrap();

        if response.status().is_success() {
            let pubkey_str = response.text().unwrap();
            let pubkey = Pubkey::from_str(&pubkey_str.trim()).unwrap();
            self.pubkey = Some(pubkey);
        } else {
            return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from(
                "Failed to retrieve pubkey",
            )));
        }

        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from(
                    "Some error message for V0_0_1",
                )));
            }
            ReplicaAccountInfoVersions::V0_0_2(_) => {
                return Err(GeyserPluginError::Custom(Box::<dyn Error + Send + Sync>::from(
                    "Some error message for V0_0_2",
                )));
            }
            ReplicaAccountInfoVersions::V0_0_3(account_info) => account_info,
        };

        if let Some(pubkey) = &self.pubkey {
            if account_info.pubkey != pubkey.to_bytes() {
                return Ok(());
            }
        } else {
            // Pubkey not retrieved yet, skip processing
            return Ok(());
        }

        let pk = Pubkey::new(account_info.pubkey);
        println!("account {:#?} updated at slot {}!", pk, slot);

        // Create a JSON payload with the account information
        let payload = serde_json::json!({
            "pubkey": account_info.pubkey,
            "slot": slot,
        });

        // Make an HTTP POST request to the specified URL
        let client = Client::new();
        let response = client
            .post("http://localhost:3000/account")
            .json(&payload)
            .send();

        // Handle the response (optional)
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Account update posted successfully!");
                } else {
                    println!("Failed to post account update: {:?}", response);
                }
            }
            Err(err) => {
                println!("Failed to post account update: {:?}", err);
            }
        }

        Ok(())
    }

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true // process account changes
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false // don't process new txs
    }
}
