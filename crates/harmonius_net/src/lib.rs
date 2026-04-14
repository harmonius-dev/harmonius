//! Harmonius networking: transport, replication, prediction, RPC, lag compensation, session
//! services, replay, and communication primitives.
//!
//! Transport and replication building blocks align with `docs/design/networking/network-transport.md`.
//! Session, replay, and communication types align with `docs/design/networking/network-services.md`.

#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod communication;
pub mod lag_comp;
pub mod prediction;
pub mod replication;
pub mod replay;
pub mod rpc;
pub mod session;
pub mod transport;
