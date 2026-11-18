# Spirit Daemon

The daemon is the network, safety, and possession authority.

Initial implementation targets Python 3.12 with:

- OSC reception
- MQTT 3.1.1/5.0 client
- Per-node joint calibration
- Lease and heartbeat management
- Pose interpolation and slew limiting
- Prometheus-compatible metrics

## Vertical-slice status

The current implementation provides packet construction and validation helpers.
It also exposes a bounded, local pose loop, but does not claim live OSC or MQTT
actuation until transport dependencies are added and lease handoff is tested.
