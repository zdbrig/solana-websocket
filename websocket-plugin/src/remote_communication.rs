use solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoV3;
use serde_json::json;
use solana_program::pubkey::Pubkey;
use reqwest::StatusCode;
use std::thread;

pub fn send_data(account_info: &ReplicaAccountInfoV3, slot: u64) {
    let pk = Pubkey::new(account_info.pubkey);

    let payload = json!({
        "pubkey": pk.to_string(),
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
