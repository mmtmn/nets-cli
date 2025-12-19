pub trait WalletAdapter {
    fn bind_agent(&mut self, agent_id: &str, wallet: &str);
    fn credit(&mut self, agent_id: &str, amount: u64);
    fn slash(&mut self, agent_id: &str, amount: u64);
    fn balance(&self, wallet: &str) -> u64;
}
