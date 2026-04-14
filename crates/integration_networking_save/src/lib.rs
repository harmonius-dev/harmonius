//! Networking ↔ save integration primitives (IR-4.6).
//!
//! This crate hosts rkyv wire types, the `SaveRpcBridge` MPSC queue, and the
//! `CloudIoRequest` submission envelope described in
//! `docs/design/integration/networking-save-system.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(test, allow(unsafe_code))]

mod bridge;
mod cloud_io;
mod wire;

pub use bridge::{SaveRpcBridge, SaveRpcMsg, SAVE_RPC_BRIDGE_CAPACITY};
pub use cloud_io::{
    cloud_io_channel, try_submit_cloud_io, CloudIoReceiver, CloudIoRequest, CloudIoSender,
    CLOUD_IO_REQUEST_CAPACITY,
};
pub use wire::{
    CheckpointAbortReason, CheckpointAbortRpc, CheckpointPrepareRpc, CheckpointReadyRpc,
    ConnectionId, SaveError, SaveRequestRpc, SaveResponseRpc, SaveRpcResult, SaveSlotMeta,
    SaveType, SchemaVersion, SlotId,
};

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use rkyv::ser::serializers::AllocSerializer;
    use rkyv::{Archive, Deserialize, Serialize};

    use super::*;

    fn rkyv_round_trip<T>(value: &T) -> T
    where
        T: Archive + Serialize<AllocSerializer<8192>>,
        T::Archived: Deserialize<T, rkyv::Infallible>,
    {
        let bytes = rkyv::to_bytes::<_, 8192>(value).expect("serialize");
        let archived = unsafe { rkyv::archived_root::<T>(&bytes[..]) };
        archived
            .deserialize(&mut rkyv::Infallible)
            .expect("deserialize")
    }

    #[test]
    fn tc_ir_4_6_u1_save_request_rpc_rkyv_round_trip() {
        let value = SaveRequestRpc {
            slot_id: SlotId(42),
            save_type: SaveType::Manual,
        };
        assert_eq!(&rkyv_round_trip(&value), &value);
    }

    #[test]
    fn tc_ir_4_6_u2_save_response_rpc_rkyv_round_trip() {
        let meta = SaveSlotMeta {
            slot_id: SlotId(9),
            timestamp: 1_700_000_000,
            content_hash: [7u8; 32],
            schema_version: SchemaVersion(12),
        };
        let value = SaveResponseRpc {
            slot_id: SlotId(9),
            result: SaveRpcResult::Success { meta },
        };
        assert_eq!(&rkyv_round_trip(&value), &value);
    }

    #[test]
    fn tc_ir_4_6_u3_checkpoint_prepare_rpc_rkyv_round_trip() {
        let value = CheckpointPrepareRpc {
            checkpoint_id: 1001,
            participants: vec![ConnectionId(1), ConnectionId(2), ConnectionId(3)],
        };
        assert_eq!(&rkyv_round_trip(&value), &value);
    }

    #[test]
    fn tc_ir_4_6_u4_checkpoint_abort_rpc_rkyv_round_trip() {
        let value = CheckpointAbortRpc {
            checkpoint_id: 55,
            reason: CheckpointAbortReason::PeerDisconnect,
        };
        assert_eq!(&rkyv_round_trip(&value), &value);
    }

    #[test]
    fn tc_ir_4_6_u5_save_rpc_bridge_fifo_single_producer() {
        let bridge = SaveRpcBridge::new();
        let a = SaveRequestRpc {
            slot_id: SlotId(1),
            save_type: SaveType::Manual,
        };
        let b = SaveRequestRpc {
            slot_id: SlotId(2),
            save_type: SaveType::Quicksave,
        };
        let c = SaveRequestRpc {
            slot_id: SlotId(3),
            save_type: SaveType::Autosave,
        };
        bridge
            .try_push(SaveRpcMsg::Request {
                who: ConnectionId(10),
                rpc: a.clone(),
            })
            .unwrap();
        bridge
            .try_push(SaveRpcMsg::Request {
                who: ConnectionId(11),
                rpc: b.clone(),
            })
            .unwrap();
        bridge
            .try_push(SaveRpcMsg::Request {
                who: ConnectionId(12),
                rpc: c.clone(),
            })
            .unwrap();
        let drained = bridge.drain();
        assert_eq!(
            drained,
            vec![
                SaveRpcMsg::Request {
                    who: ConnectionId(10),
                    rpc: a,
                },
                SaveRpcMsg::Request {
                    who: ConnectionId(11),
                    rpc: b,
                },
                SaveRpcMsg::Request {
                    who: ConnectionId(12),
                    rpc: c,
                },
            ]
        );
    }

    #[test]
    fn tc_ir_4_6_u6_save_rpc_bridge_multi_producer() {
        let bridge = SaveRpcBridge::new();
        let tx_a = bridge.sender();
        let tx_b = bridge.sender();
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let b1 = barrier.clone();
        let b2 = barrier.clone();
        let t1 = thread::spawn(move || {
            b1.wait();
            for i in 0..100 {
                let msg = SaveRpcMsg::CheckpointReady {
                    who: ConnectionId(1),
                    checkpoint_id: i,
                };
                loop {
                    if tx_a.try_send(msg.clone()).is_ok() {
                        break;
                    }
                    thread::yield_now();
                }
            }
        });
        let t2 = thread::spawn(move || {
            b2.wait();
            for i in 0..100 {
                let msg = SaveRpcMsg::CheckpointReady {
                    who: ConnectionId(2),
                    checkpoint_id: i + 1_000,
                };
                loop {
                    if tx_b.try_send(msg.clone()).is_ok() {
                        break;
                    }
                    thread::yield_now();
                }
            }
        });
        barrier.wait();
        let mut drained = Vec::new();
        while drained.len() < 200 {
            drained.extend(bridge.drain());
            thread::yield_now();
        }
        t1.join().unwrap();
        t2.join().unwrap();
        drained.extend(bridge.drain());
        assert_eq!(drained.len(), 200);
        drained.sort_by_key(|msg| match msg {
            SaveRpcMsg::CheckpointReady { checkpoint_id, .. } => *checkpoint_id,
            SaveRpcMsg::Request { .. } => u64::MAX,
        });
        let mut seen_a = 0u32;
        let mut seen_b = 0u32;
        for msg in &drained {
            match msg {
                SaveRpcMsg::CheckpointReady { who, checkpoint_id } => match who.0 {
                    1 => {
                        assert_eq!(*checkpoint_id, u64::from(seen_a));
                        seen_a += 1;
                    }
                    2 => {
                        assert_eq!(*checkpoint_id, u64::from(seen_b) + 1_000);
                        seen_b += 1;
                    }
                    _ => panic!("unexpected producer id"),
                },
                SaveRpcMsg::Request { .. } => panic!("unexpected message"),
            }
        }
        assert_eq!(seen_a, 100);
        assert_eq!(seen_b, 100);
    }

    #[test]
    fn tc_ir_4_6_u7_cloud_io_request_enum_sizing() {
        assert!(std::mem::size_of::<CloudIoRequest>() <= 32);
    }

    #[test]
    fn tc_ir_4_6_n1_save_rpc_bridge_full_returns_io() {
        let bridge = SaveRpcBridge::new();
        for i in 0..SAVE_RPC_BRIDGE_CAPACITY {
            bridge
                .try_push(SaveRpcMsg::CheckpointReady {
                    who: ConnectionId(i as u64),
                    checkpoint_id: i as u64,
                })
                .unwrap();
        }
        let err = bridge.try_push(SaveRpcMsg::CheckpointReady {
            who: ConnectionId(999),
            checkpoint_id: 999,
        });
        assert_eq!(err, Err(SaveError::Io));
    }

    #[test]
    fn tc_ir_4_6_n2_cloud_io_request_channel_full() {
        let (tx, _rx) = cloud_io_channel();
        for _ in 0..CLOUD_IO_REQUEST_CAPACITY {
            try_submit_cloud_io(&tx, CloudIoRequest::Query { slot: SlotId(0) }).unwrap();
        }
        let err = try_submit_cloud_io(&tx, CloudIoRequest::Query { slot: SlotId(1) });
        assert_eq!(err, Err(SaveError::CloudUnavailable));
    }
}
