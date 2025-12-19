use std::collections::HashMap;
use std::fs;

use serde::{Serialize, Deserialize};

use super::adapter::WalletAdapter;

#[derive(Default, Serialize, Deserialize)]
pub struct MockWalletAdapter {
    agent_to_wallet: HashMap<String, String>,
    wallet_balances: HashMap<String, u64>,
}

impl WalletAdapter for MockWalletAdapter {
    fn bind_agent(&mut self, agent_id: &str, wallet: &str) {
        self.agent_to_wallet.insert(agent_id.to_string(), wallet.to_string());
        self.wallet_balances.entry(wallet.to_string()).or_insert(0);
    }

    fn credit(&mut self, agent_id: &str, amount: u64) {
        if let Some(wallet) = self.agent_to_wallet.get(agent_id) {
            *self.wallet_balances.entry(wallet.clone()).or_insert(0) += amount;
        }
    }

    fn slash(&mut self, agent_id: &str, amount: u64) {
        if let Some(wallet) = self.agent_to_wallet.get(agent_id) {
            let entry = self.wallet_balances.entry(wallet.clone()).or_insert(0);
            *entry = entry.saturating_sub(amount);
        }
    }

    fn balance(&self, wallet: &str) -> u64 {
        *self.wallet_balances.get(wallet).unwrap_or(&0)
    }
}

impl MockWalletAdapter {
    pub fn save(&self, path: &str) {
        let mut json: serde_json::Value =
            fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_else(|| serde_json::json!({}));

        json["wallets"] = serde_json::to_value(&self.wallet_balances).unwrap();
        json["bindings"] = serde_json::to_value(&self.agent_to_wallet).unwrap();

        fs::write(path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
    }

    pub fn load(path: &str) -> Self {
        let mut out = MockWalletAdapter::default();
        if let Ok(s) = fs::read_to_string(path) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                if let Some(w) = v.get("wallets") {
                    out.wallet_balances =
                        serde_json::from_value(w.clone()).unwrap_or_default();
                }
                if let Some(b) = v.get("bindings") {
                    out.agent_to_wallet =
                        serde_json::from_value(b.clone()).unwrap_or_default();
                }
            }
        }
        out
    }

    pub fn all_wallets(&self) -> &HashMap<String, u64> {
        &self.wallet_balances
    }
}
