use nets::{
    agent::Agent,
    chess::system::ChessObservation,
    chess::r#move::ChessMove,
    wasm_agent::WasmAgent,
};

pub struct ChessWasmAgent {
    inner: WasmAgent,
}

impl ChessWasmAgent {
    pub fn load(agent_id: String, wasm: &[u8]) -> Self {
        let inner = WasmAgent::load(agent_id, wasm)
            .expect("failed to load wasm agent");
        Self { inner }
    }
}

fn u64_to_chess_move(v: u64) -> ChessMove {
    let from = (v & 0b111111) as u8;
    let to = ((v >> 6) & 0b111111) as u8;
    let promo = ((v >> 12) & 0b1111) as u8;
    ChessMove { from, to, promotion: promo }
}

impl Agent<ChessObservation, ChessMove> for ChessWasmAgent {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn decide(&mut self, _obs: ChessObservation) -> ChessMove {
        u64_to_chess_move(self.inner.decide(0))
    }
}
