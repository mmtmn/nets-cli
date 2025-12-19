use std::fs;
use std::path::Path;

use nets::{
    system::System,
    agent::Agent,
    snake::{SnakeSystem, Dir},
    wasm_agent::WasmAgent,
    r#match::run_match,
};

/* ------------------------------
   Action decoding
-------------------------------*/

fn u64_to_dir(v: u64) -> Dir {
    match v % 4 {
        0 => Dir::Up,
        1 => Dir::Down,
        2 => Dir::Left,
        _ => Dir::Right,
    }
}

/* ------------------------------
   WASM Snake Adapter
-------------------------------*/

struct SnakeWasmAgent {
    inner: WasmAgent,
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

/* ------------------------------
   nets verify
-------------------------------*/

pub fn verify(agent: String) {
    let agents_dir = Path::new("agents");
    let wasm_path = agents_dir.join(format!("{}.wasm", agent));

    if !wasm_path.exists() {
        eprintln!("agent wasm not found: {:?}", wasm_path);
        std::process::exit(1);
    }

    let wasm = fs::read(&wasm_path).expect("failed to read wasm");

    let mut agent_a = SnakeWasmAgent {
        inner: WasmAgent::load(agent.clone(), &wasm)
            .expect("failed to load agent"),
    };

    let mut agent_b = SnakeWasmAgent {
        inner: WasmAgent::load(agent.clone(), &wasm)
            .expect("failed to load agent"),
    };

    let system = SnakeSystem::new(10, 10, 300);

    let result_a = run_match(system.clone(), &mut agent_a);
    let result_b = run_match(system.clone(), &mut agent_b);

    if result_a.score != result_b.score {
        eprintln!(
            "verification failed: score mismatch ({} vs {})",
            result_a.score,
            result_b.score
        );
        std::process::exit(1);
    }

    println!(
        "verification ok: agent={} score={}",
        agent,
        result_a.score
    );
}
