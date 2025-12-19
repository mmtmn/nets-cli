use nets::{ledger::Ledger, league_state::LeagueState, persist};

pub fn show(agent: Option<String>) {
    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();

    persist::load("state.json", &mut ledger, &mut league_state);

    match agent {
        Some(a) => {
            let bal = ledger.balance(&a);
            println!("{} balance={}", a, bal);
        }
        None => {
            println!("balances:");
            for (id, bal) in ledger.snapshot() {
                let status = if league_state.is_disqualified(&id) {
                    " (disqualified)"
                } else {
                    ""
                };

                println!("{} balance={}{}", id, bal, status);
            }
        }
    }
}
