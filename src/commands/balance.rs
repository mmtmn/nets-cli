use nets::{ledger::Ledger, league_state::LeagueState, persist};

use crate::wallet::mock::MockWalletAdapter;
use crate::wallet::adapter::WalletAdapter;

pub fn show(agent: Option<String>, wallet: Option<String>) {
    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();
    let _ = persist::load("state.json", &mut ledger, &mut league_state);

    let wallet_adapter = MockWalletAdapter::load("state.json");

    if let Some(a) = agent {
        println!("{} balance={}", a, ledger.balance(&a));
        return;
    }

    if let Some(w) = wallet {
        println!("wallet {} balance={}", w, wallet_adapter.balance(&w));
        return;
    }

    println!("agent balances:");
    for (id, bal) in ledger.snapshot() {
        println!("{} balance={}", id, bal);
    }

    println!("\nwallet balances:");
    for (w, b) in wallet_adapter.all_wallets() {
        println!("{} balance={}", w, b);
    }
}
