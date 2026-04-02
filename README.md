# tsp-types

Rust type definitions for the [Type Server Protocol (TSP)](https://github.com/nicolo-ribaudo/tc39-proposal-type-server-protocol) — structured type queries between type checkers and editor extensions over JSON-RPC.

## Repository structure

```
tsp-types/
├── protocol/                    # TypeScript protocol definition (source of truth)
│   ├── typeServerProtocol.ts    # Canonical TSP interface definitions
│   ├── tspSupplemental.ts       # Supplemental TypeScript types
│   ├── tsp.json                 # Machine-readable JSON representation
│   ├── tsp.schema.json          # JSON schema for tsp.json validation
│   └── generate_json.py         # TS → JSON generator
├── generator/                   # Rust code generator
│   └── generate_protocol.py     # JSON → Rust generator (uses lsprotocol)
├── src/                         # Rust crate source
│   ├── lib.rs
│   ├── protocol.rs              # Method constants, protocol version
│   ├── types.rs                 # Type model (Type, TypeKind, TypeDetails, etc.)
│   └── requests.rs              # Request/response parameter types
├── Cargo.toml
└── LICENSE
```

## Updating the protocol

When the TypeScript protocol definition changes upstream:

1. **Copy** the updated `typeServerProtocol.ts` (and `tspSupplemental.ts` if changed) from the [pyrx](https://github.com/nicolo-ribaudo/tc39-proposal-type-server-protocol) repository into `protocol/`.

2. **Regenerate** the JSON representation:
   ```sh
   cd protocol
   python generate_json.py
   ```

3. **Regenerate** the Rust types (requires [uv](https://docs.astral.sh/uv/) or a venv with lsprotocol):
   ```sh
   cd generator
   uv run generate_protocol.py
   ```

4. **Review** the generated `src/protocol.rs`, reconcile with any hand-written code in `src/types.rs` and `src/requests.rs`, then run tests:
   ```sh
   cargo test
   ```

5. **Commit** the updated protocol files and Rust source.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
tsp-types = "0.4"
```

## License

MIT
