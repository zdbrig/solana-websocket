use std::sync::Arc;
use std::sync::Mutex;
use solana_program::pubkey::Pubkey;
use reqwest::Client;
use std::str::FromStr;
use std::time::Duration;
use reqwest::StatusCode;
use crate::constants::PUBKEY_URL;
use crate::constants::REFRESH_INTERVAL;

pub async fn start_refresher(pubkeys: Arc<Mutex<Vec<Option<Pubkey>>>>) {
    let client = Client::new();

    loop {
        tokio::time::sleep(Duration::from_secs(REFRESH_INTERVAL)).await;

        let response = client.get(PUBKEY_URL).send().await;

        if let Ok(response) = response {
            if response.status().is_success() {
                if let Ok(pubkeys_str) = response.text().await {
                    let pubkeys_str = pubkeys_str.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();

                    let mut pubkeys_guard = pubkeys.lock().unwrap();
                    pubkeys_guard.clear();
                    for pubkey_str in pubkeys_str {
                        if let Ok(new_pubkey) = Pubkey::from_str(&pubkey_str) {
                            pubkeys_guard.push(Some(new_pubkey));
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
