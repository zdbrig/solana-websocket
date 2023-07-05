use super::refresh::start_refresher;
use std::sync::Arc;
use std::sync::Mutex;
use solana_program::pubkey::Pubkey;

#[derive(Debug)]
pub struct SimplePlugin {
    pub pubkeys: Arc<Mutex<Vec<Option<Pubkey>>>>, // pubkeys should be public
}

impl Default for SimplePlugin {
    fn default() -> Self {
        let pubkeys = Arc::new(Mutex::new(Vec::new()));
        let cloned_pubkeys = Arc::clone(&pubkeys);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                start_refresher(cloned_pubkeys).await;
            });
        });

        SimplePlugin { pubkeys }
    }
}
