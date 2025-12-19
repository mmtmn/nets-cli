use nets::{
    agent::Agent,
    rps::system::RpsObservation,
    rps::r#move::RpsMove,
    wasm_agent::WasmAgent,
};

pub struct RpsWasmAgent {
    inner: WasmAgent,
}

impl RpsWasmAgent {
    pub fn load(agent_id: String, wasm: &[u8]) -> Self {
        let inner = WasmAgent::load(agent_id, wasm)
            .expect("failed to load wasm agent");
        Self { inner }
    }
}

impl Agent<RpsObservation, RpsMove> for RpsWasmAgent {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn decide(&mut self, _obs: RpsObservation) -> RpsMove {
        RpsMove::from_u64(self.inner.decide(0))
            .unwrap_or(RpsMove::Rock)
    }
}
