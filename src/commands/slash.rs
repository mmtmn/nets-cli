use nets::{ledger::Ledger, league_state::LeagueState, persist};

use crate::wallet::mock::MockWalletAdapter;
use crate::wallet::adapter::WalletAdapter;

pub fn slash(agent: String, amount: u64) {
    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();

    let _ = persist::load("state.json", &mut ledger, &mut league_state);

    let mut wallet_adapter = MockWalletAdapter::load("state.json");

    ledger.slash(&agent, amount);
    wallet_adapter.slash(&agent, amount);

    persist::save("state.json", &ledger, &league_state, Vec::new());
    wallet_adapter.save("state.json");

    println!(
        "slashed agent {} by {}",
        agent, amount
    );
}
