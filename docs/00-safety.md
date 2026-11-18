# Safety

## Hazard classes

| Hazard | Primary controls |
| --- | --- |
| Crushed fingers or clothing | Reduced-torque starters, guarded pinch zones, current limit, emergency stop |
| Falling humanoid | Low center of gravity, locking caster, tether, heel/foot outriggers |
| Li-ion/LiFePO4 fire | Protected pack, fuse at source, documented pack chemistry, charge outside occupied room |
| 24 V wiring fault | Ferrules, strain relief, fusing per rail, over-current protection |
| Servo runaway | MCU watchdog, valid lease timeout, independent relay, position limits |
| Tether snag / moving robot | Dual-line tether, stop zone, no autonomous walking on first prototype |
| Sharp/overheated prints | Fillet/guard moving edges, ventilate enclosure, thermal shutdown |

## Mandatory controls

- A wired emergency stop shall remove actuator power at the battery side.
- An e-stop must use a normally-open-to-close reset circuit and must not depend
  on the MCU or broker.
- Keep the initial desk bust and bench prototypes below 25 kg.
- Do not build a walking gait until a tethered, externally braked stationary
  prototype passes 100 repeated safety tests.
- Every actuator has a software and mechanical limit.
- Every node has a watchdog and enters a known doll pose after communication loss.
- No node may start movement while the MQTT broker has no valid lease.
- Keep all moving joints outside the 0-100 mm interaction zone around a person's
  face and upper chest until proximity sensors and validated low-force modes exist.
- Use only a power supply, battery management system, and wiring rated for the
  measured stall and continuous currents.

## Build phases

1. Desktop actuator bench
2. Head face/gimbal with power off to humans
3. Stationary desk bust
4. Foyer pose-only mannequin
5. Foyer mobile platform
6. Kitchen and arm workcells only after the safety case passes
