# Software Architecture

## Canonical pose

Canonical targets use normalized or SI units:

```json
{
  "sequence": 123,
  "lease_id": "kitchen-02",
  "expires_at_ms": 189,
  "joints": {
    "head_yaw": 0.10,
    "head_pitch": -0.05,
    "jaw": 0.20,
    "left_brow": 0.40,
    "right_brow": 0.40,
    "left_eye_x": 0.03,
    "left_eye_y": -0.08
  }
}
```

All normalized joint values are in the inclusive range [-1, 1] except explicit
blend amounts, which are [0, 1].

## MQTT topics

| Topic | Direction | Purpose |
| --- | --- | --- |
| `spirit/request/<node>` | Daemon -> broker | Lease request |
| `spirit/lease/<node>` | Node -> broker | Lease acknowledgement |
| `spirit/pose/<node>` | Daemon -> broker | Canonical pose |
| `spirit/status/<node>` | Node -> broker | Heartbeat and telemetry |
| `spirit/fault/<node>` | Node -> broker | Fault event |
| `spirit/e_stop/<node>` | Broker -> node | Latched stop (if network independent) |

## Lease rules

- Lease default: 2 seconds.
- Motion packet maximum age: 500 ms.
- Heartbeat every 250 ms.
- A node with expired lease immediately returns to safe pose.
- Relay power remains off until a valid lease and start sequence complete.
