# Requirements

## Functional requirements

FR-01: A single central spirit identity may own exactly one physical node at a time.
FR-02: Any node owned by the spirit shall animate at 30 Hz nominally.
FR-03: Motion shall transition into a calibrated safe pose when telemetry stops.
FR-04: The daemon shall reject poses outside joint limits.
FR-05: A node shall expose status, fault, voltage, temperature, and lease time.
FR-06: A node shall not require VRChat connectivity to enter sleep mode.
FR-07: Home Assistant may trigger possession handoff but may not directly command joint targets.
FR-08: The system shall support face, neck, torso, arm, and hand DoF as optional modules.
FR-09: All printable parts shall include their units and print orientation.
FR-10: The head shall remain mechanically isolated from actuator motor torque.

## Nonfunctional requirements

- P95 telemetry latency under 100 ms on a local wired or Wi-Fi network.
- No servo command louder or faster than calibrated slew limits.
- Controller boot to safe state under 3 seconds.
- Packet parsing bounded and deterministic.
- Firmware flash and configuration recoverable over USB.
- CAD parameters centralized in `hardware/mechanical/config.scad`.

## Acceptance criteria

| ID | Test |
| --- | --- |
| AC-01 | Unplugging network returns every node to sleep within 2 seconds |
| AC-02 | Losing broker connection cuts actuator current |
| AC-03 | Out-of-range targets are rejected and logged |
| AC-04 | Two nodes cannot possess the spirit simultaneously |
| AC-05 | One node has no movement after lease expiry |
| AC-06 | Parts fit specified 220 mm build plate with orientation marker |
| AC-07 | Assembly can be disassembled without destructive drilling |
