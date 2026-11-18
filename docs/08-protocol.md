# Protocol

## OSC input

The VRChat bridge emits a flat OSC payload to the daemon. Physical-facing topics
do not expose OSC addresses.

Example:

```text
/avatar/parameters/SpiritHeadYaw        0.10
/avatar/parameters/SpiritHeadPitch     -0.05
/avatar/parameters/SpiritJaw           0.20
```

The daemon remaps VRChat animation names to canonical joints.

## MQTT JSON

Motion JSON must include a monotonically increasing sequence, lease ID, expiry,
and a schema version.

```json
{
  "v": 1,
  "seq": 123,
  "lease": "kitchen-02",
  "node": "kitchen-02",
  "expires_ms": 500,
  "joints": {
    "head_yaw": 0.10,
    "head_pitch": -0.05
  }
}
```

`node` is the absolute target node ID. `lease` is the possession token and must
match the daemon-issued lease for `node`.

`expires_ms` is a relative packet lifetime from 1 to 500 ms. The node converts
it to an absolute `expires_at_ms` deadline using its own `millis()` clock.
The binary packet also carries `protocol_version`, `node_id`, `lease_id`,
monotonic sequence, and an absolute monotonic expiry. Firmware clocks shall not
use wall-clock or broker timestamps.

## Validation rules

- Reject unknown schema versions.
- Reject sequence regression or wrap without node reset.
- Reject missing or unknown lease.
- Reject invalid JSON or values outside [-1, 1].
- Clamp each pose to node-specific limits.
- Apply daemonic, not MCU, command validation.
