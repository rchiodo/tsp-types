//! Type Server Protocol (TSP) definitions.
//!
//! This crate defines the Rust types that correspond to the TSP protocol
//! defined in `typeServerProtocol.ts`. These are used for JSON-RPC
//! communication between a TSP client (e.g., Pylance) and a type checker
//! (e.g., ty, pyrefly).
//!
//! The crate is organized into:
//! - [`protocol`]: Method name constants and protocol version
//! - [`types`]: Type representations sent over the wire
//! - [`requests`]: Request/response parameter types

pub mod protocol;
pub mod requests;
pub mod types;
