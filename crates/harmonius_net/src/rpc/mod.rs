//! RPC dispatch, validation, multicast, and reliability coalescing.

pub mod dispatcher;

pub use dispatcher::{
    cast_ability, merge_reliable_latest, multicast_spatial, multicast_spatial_disk,
    rpc_param_checks, CastAbility, Chat, DamageNumber, RpcError, TradeOffer,
};
