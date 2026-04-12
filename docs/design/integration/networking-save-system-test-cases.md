# Networking ↔ Save System Integration Test Cases

## Integration Tests

| ID | Test | Input | Expected | Req |
|----|------|-------|----------|-----|
| TC-IR-4.6.1.1 | Server-only save trigger | Server calls trigger_save | Save completes | IR-4.6.1 |
| TC-IR-4.6.1.2 | Client cannot save directly | Client calls trigger_save | Rejected, no-op | IR-4.6.1 |
| TC-IR-4.6.2.1 | Save request RPC round-trip | Client sends SaveRequestRpc | Server responds Success | IR-4.6.2 |
| TC-IR-4.6.2.2 | Permission denied response | Unprivileged client | SaveRpcResult::PermissionDenied | IR-4.6.2 |
| TC-IR-4.6.3.1 | Cloud upload via platform API | Steam Cloud available | Upload via Steam API | IR-4.6.3 |
| TC-IR-4.6.3.2 | Cloud upload fallback to QUIC | No platform API | Upload via reliable QUIC | IR-4.6.3 |
| TC-IR-4.6.4.1 | Conflict detected | Local != remote hash | SyncResult::Conflict returned | IR-4.6.4 |
| TC-IR-4.6.4.2 | KeepLocal resolves conflict | User chooses KeepLocal | Local uploaded to cloud | IR-4.6.4 |
| TC-IR-4.6.4.3 | KeepRemote resolves conflict | User chooses KeepRemote | Remote downloaded to local | IR-4.6.4 |
| TC-IR-4.6.5.1 | Checkpoint all clients ready | 4 clients ACK | Save triggers | IR-4.6.5 |
| TC-IR-4.6.5.2 | Checkpoint timeout on disconnect | 1 client disconnects | Checkpoint aborted | IR-4.6.5 |
| TC-IR-4.6.6.1 | ConnectionId excluded from save | Save world with net entities | No ConnectionId in output | IR-4.6.6 |
| TC-IR-4.6.6.2 | Saveable components included | Save world with Saveable | All Saveable components saved | IR-4.6.6 |
| TC-IR-4.6.6.3 | PredictionState excluded | Save predicted entity | No PredictionState in output | IR-4.6.6 |

## Benchmarks

| ID | Benchmark | Target | Req |
|----|-----------|--------|-----|
| TC-IR-4.6.1.B1 | Full save serialization | < 100 ms | IR-4.6.1 |
| TC-IR-4.6.2.B1 | Save RPC round-trip latency | < 200 ms (LAN) | IR-4.6.2 |
| TC-IR-4.6.3.B1 | Cloud upload 10 MB save | < 5 s (broadband) | IR-4.6.3 |
| TC-IR-4.6.5.B1 | Checkpoint coordination 8 clients | < 500 ms | IR-4.6.5 |
