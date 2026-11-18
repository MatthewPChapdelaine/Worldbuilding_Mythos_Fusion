# Analysis: Worldbuilding_Mythos_Fusion

A critique of what's actually in the repo. Short version: **strong documentation discipline, essentially zero working code.** It's a design package masquerading (in its own defense, honestly) as a buildable system.

## What's genuinely good

**The safety doc is the best thing here.** `00-safety.md` reads like someone who has actually been hurt by a robot. Hazard table, mandatory controls, and the key insight most hobbyists miss: the e-stop must be hardware, battery-side, and independent of the MCU and broker. Also good: "no autonomous walking on first prototype," "keep the interaction zone 0–100 mm from a person's face clear until force limits exist." The build-phase ladder (bench → head → bust → mannequin → mobile) is correctly ordered.

**The lease/possession model is sound.** One spirit identity, one node at a time, no peer-to-peer motion transfer, daemon as sole authority. `AC-04` ("two nodes cannot possess the spirit simultaneously") is a real acceptance criterion, not a vibe. Handoff sequence with 500–1000 ms ramp before the source powers down is right.

**Requirements are testable.** FR-01 through FR-10, AC-01 through AC-07, P95 latency budget, bounded packet parsing. Rare for a hobby repo to write acceptance criteria at all.

**Bowden tendons for the head.** Correct call — it gets motor mass out of the skull and off the neck.

## Where it falls apart

**Every line of code is a stub.** Look at what's actually implemented:

- `daemon.py` — prints a JSON blob and `sleep(1)`. Comment literally says "TODO: Receive OSC, own lease, validate, and publish MQTT."
- `node-controller/main.cpp` — blinks `LED_BUILTIN`. "TODO: Implement MQTT lease, watchdog, PWM calibration, and relay control."
- `packet.h` — a struct with **no `lease` field and no schema version field**, despite `08-protocol.md` demanding both. So the one binary artifact that exists already contradicts its own spec. It also hardcodes `JOINT_LIMIT 100` with no node ID, so a packet doesn't say *which* node it's for.

**Protocol/transport mismatch.** The docs specify MQTT JSON. `packet.h` is a fixed binary struct. Nothing says how one becomes the other, and there's no serialization code. `expires_ms: 189` in the example is ambiguous — absolute or relative? A 189 ms absolute timestamp is meaningless, so presumably relative, but nothing states it.

**"Rust mythology worldbuilding toolkit" commit vs. no Rust.** Latest commit message says Rust; languages are Python/OpenSCAD/C++/C. Rust appears nowhere.

**The mechanical design is a five-variable scaffold.** `config.scad` is 12 lines: head 160×210×165, `$fn = 36`, done. The README's honest disclaimer ("not load-bearing or printable until loads are validated") is good, but there's no joint-load table, no factor of safety, no fastener standard, no anthropometric source. The BOM even lists "High-torque shoulder/waist actuator | N | select from measured joint-load table" — a table that doesn't exist. So the BOM can't be costed.

**Electrical doc punts the hard part.** "Wire and fuse sizing are not specified. Calculate from…" — that's the entire value of an electrical doc. No ampacity, no gauge, no converter ratings, no thermal analysis for a 25 kg class machine running servos. `06` mentions "24 V wiring fault" while the BOM implies LiFePO4 with a DC/DC to servo rail; the rail voltage is never pinned down.

**Power budget is hand-waved.** "Capacity based on 15-minute worst-case duty cycle" with no worst-case number. For a robot meant to sit in a kitchen, that's the single most important calculation and it's absent.

**Two face displays per head, 240×240, "IPS/OLED."** Cute, but a 36 mm eye display (`eye_display_size_mm`) in a 160 mm head means the render budget, refresh sync, and power draw are all unaddressed.

**No CI, no tests.** `tests/` is listed for "protocol, safety, hardware-in-the-loop" and contains nothing runnable. 0 stars, 4 commits, one hour old — fair, but it means nothing here is proven.

**Naming incoherence.** "Distributed Animatronic Android Network," "Mythos Fusion," "Spirit Daemon," "spirit identity," "doll pose." Great flavor, but the repo never defines what "mythos fusion" means or how worldbuilding relates to a physical robot. The VRChat/UdonSharp dependency is asserted, never justified — why is a VR social platform the animation authority for a physical bust?

## Priority fixes

1. Define the lease field and version in `packet.h`, or drop the struct and commit to JSON.
2. Write the joint-load table and pick the actuator class; then the BOM and CAD become real.
3. Do the power/thermal budget with a stated duty cycle and rail voltage.
4. Implement one vertical slice end to end: OSC → daemon → MQTT → one ESP32 → one servo, with lease expiry cutting the relay.
5. Add a CI job that at least parses a packet and rejects an out-of-range pose.

## Verdict

**7/10 as a design document, 2/10 as software.** It's a well-organized, safety-conscious specification that will save the author real pain later — and a long way from the "implementation-ready" claim in the README. "Implementation-ready" should mean "an experienced builder could start Thursday without asking questions." Right now the questions start at line 1: what actuator, what voltage, what budget.

*Note: this was cut short at the iteration limit. Not yet read: `docs/03-hardware-architecture.md`, `07-software-architecture.md`, `09-commissioning.md`, the `docs/nodes/` specs, `config/`, `tests/`, the VRChat and simulator directories, the `projects/loporia-koretoretos` subtree, or the long "I want to 3D Print a Robot Maid…" markdown. Those would likely sharpen or soften the code and mechanical criticism — the node specs in particular could resolve the packet-targeting problem. Tell me to continue and I'll finish the sweep.*
