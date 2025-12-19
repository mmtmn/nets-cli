use nets::{
    system::System,
    agent::Agent,
    snake::{SnakeSystem, Dir},
    wasm_agent::WasmAgent,
};

pub struct SnakeWasmAgent {
    inner: WasmAgent,
}

impl SnakeWasmAgent {
    pub fn load(agent_id: String, wasm: &[u8]) -> Self {
        let inner = WasmAgent::load(agent_id, wasm)
            .expect("failed to load wasm agent");
        Self { inner }
    }
}

fn u64_to_dir(v: u64) -> Dir {
    match v % 4 {
        0 => Dir::Up,
        1 => Dir::Down,
        2 => Dir::Left,
        _ => Dir::Right,
    }
}

impl Agent<
    <SnakeSystem as System>::Observation,
    <SnakeSystem as System>::Action,
> for SnakeWasmAgent {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn decide(
        &mut self,
        obs: <SnakeSystem as System>::Observation,
    ) -> <SnakeSystem as System>::Action {
        let ((hx, hy), (ax, ay), _) = obs;

        let packed =
            ((hx as u64 & 0xFF) << 24)
            | ((hy as u64 & 0xFF) << 16)
            | ((ax as u64 & 0xFF) << 8)
            | (ay as u64 & 0xFF);

        u64_to_dir(self.inner.decide(packed))
    }
}
