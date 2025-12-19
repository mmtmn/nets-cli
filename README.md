# nets-cli

Command-line interface for running, verifying, and managing agents in **nets-core** competitive intelligence markets.

nets-cli is a thin UX layer.  
All rules, economics, and determinism live in nets-core.

---

## What nets-cli Does

- Run local deterministic competitions
- Commit results to persistent state
- Show balances and capital
- Verify agent determinism via replay
- Build and manage WASM agents

nets-cli never defines rules. It only executes them.

---

## Installation

From the repo root:

```bash
cargo install --path .
````

This installs the `nets` command into your PATH.

---

## Basic Usage

### Run a league (dry run)

```bash
nets run --matches 5
```

### Run and commit results

```bash
nets run --matches 5 --commit
```

### Show balances

```bash
nets balance
nets balance --agent agent_a
```

### Verify an agent

```bash
nets verify --agent agent_a
```

### Build a guest WASM agent

```bash
nets agent build --path agents/guest_snake_agent
```

---

## Directory Layout

```
nets-cli/
├── agents/           # compiled .wasm agents (not committed)
├── state.json        # persistent local state
└── src/
```

Agents are discovered automatically from `agents/*.wasm`.

---

## Relationship to nets-core

* nets-core defines:

  * systems
  * execution
  * verification
  * slashing
  * economics

* nets-cli provides:

  * UX
  * persistence
  * local workflows

nets-cli must never change protocol semantics.

---

## Philosophy

nets-cli exists to make nets usable without weakening it.

If something feels “convenient but unsafe”, it doesn’t belong here.