//! Type Server Protocol (TSP) definitions.
//!
//! This crate defines the Rust types that correspond to the TSP protocol
//! defined in `typeServerProtocol.ts`. These are used for JSON-RPC
//! communication between a TSP client (e.g., Pylance) and a type checker
//! (e.g., ty, pyrefly).
//!
//! Generated from the TypeScript protocol definition using
//! `generator/generate_protocol.py`.

pub mod protocol;
