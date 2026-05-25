# plato-engine

High-performance PLATO tile engine — a Rust replacement for the Python PLATO server core. Extracted from [forgemaster/plato-engine](https://github.com/SuperInstance/forgemaster), now a standalone Cocapn fleet component.

## What It Does

plato-engine provides the core tile processing pipeline for PLATO rooms: submission, quality gating, storage, retrieval, and cross-room pathfinding. It's a drop-in Rust replacement for the Python PLATO server, designed for high-throughput fleet deployments where every microsecond counts.

The engine handles tile lifecycle (submit → gate → store → query), room management, and inter-room navigation via tag-based adjacency graphs.

## Architecture

```
Tile → Gate (quality checks) → Room (storage) → Query (retrieval)
                                      ↓
                               Pathfinder (cross-room routing)
                                      ↓
                               Server (HTTP API, :8847)
```

### Core Modules

| Module | Purpose |
|--------|---------|
| `tile` | Tile data model — `Tile`, `TileBuilder`, `Provenance`, content hashing |
| `room` | Room management — named tile collections with hash-indexed dedup |
| `gate` | Quality gate — rejects absolute claims, duplicates, and malformed tiles |
| `engine` | Top-level `PlatoEngine` — coordinates rooms, stats, health checks |
| `pathfinder` | Cross-room routing via tag-based adjacency + BFS shortest path |
| `server` | Axum HTTP server — REST API for tile submission and queries |

### Key Types

```rust
use plato_engine::{Tile, Room, Gate, PlatoEngine, Pathfinder};

// Build a tile
let tile = Tile::builder()
    .domain("harbor")
    .question("What coordinates the fleet?")
    .answer("The harbor — a coordination hub.")
    .confidence(0.95)
    .build();

// Submit through the engine (includes gate check)
let engine = PlatoEngine::with_defaults();
engine.submit("my-room", tile)?;

// Query room health
let health = engine.health();  // rooms_count, tiles_count, uptime, stats

// Find paths between rooms
let pathfinder = Pathfinder::new(&engine);
let path = pathfinder.find_path("forgemaster-anvil", "harbor");
```

## Server

```bash
# Run the HTTP server
cargo run --bin plato-engine

# Custom port
PLATO_PORT=9000 cargo run --bin plato-engine
```

Default port: `8847` (compatible with Python PLATO server).

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/submit` | Submit a tile to a room |
| GET | `/rooms` | List all rooms |
| GET | `/rooms/{id}` | Get room details + tiles |
| GET | `/health` | Engine health + stats |
| POST | `/query` | Query tiles across rooms |

## Quality Gate

The gate rejects tiles that:
- Contain absolute claims ("always", "never", "guaranteed")
- Are duplicates (content hash match)
- Have missing required fields
- Are below minimum length thresholds

Gate behavior is configurable via `GateConfig`.

## Benchmarks

```bash
cargo bench
```

Benchmarks cover gate evaluation, tile hashing, room insertion, and pathfinder queries.

## Origin

Originally developed as part of the [forgemaster](https://github.com/SuperInstance/forgemaster) monorepo. Extracted for independent deployment as a fleet component in the Cocapn ecosystem.

## Related Repos

- **[forgemaster](https://github.com/SuperInstance/forgemaster)** — Original monorepo, forge orchestration
- **[plato-core](https://github.com/SuperInstance/plato-core)** — Foundation types and mesh registry (Python)
- **[cocapn-plato](https://github.com/SuperInstance/cocapn-plato)** — Full Cocapn PLATO integration (Python SDK + server)
- **[plato-mcp](https://github.com/SuperInstance/plato-mcp)** — PLATO rooms as MCP tools
- **[cocapn-glue-core](https://github.com/SuperInstance/cocapn-glue-core)** — Binary wire protocol for fleet communication

## License

MIT
