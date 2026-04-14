//! Harmonius networking: transport, replication, prediction, RPC, and lag compensation.
//!
//! This crate provides deterministic, testable building blocks aligned with
//! `docs/design/networking/network-transport.md`. Platform I/O and QUIC drivers are out of scope
//! here; callers integrate via the sans-io style types in [`transport`].

#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod lag_comp;
pub mod prediction;
pub mod replication;
pub mod rpc;
pub mod transport;
