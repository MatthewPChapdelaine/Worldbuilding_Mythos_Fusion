# Mechanical Design

## Units and conventions

- All CAD dimensions use millimeters.
- Positive Z is up.
- Positive X points to the robot's right.
- Positive yaw turns right-facing toward positive X.
- Print tolerances use a 0.25 mm clearance baseline, then calibrate a cube.
- Fastener holes use 0.2 mm print compensation unless hardware data requires more.

## Shared skeleton

The initial mechanical spine uses replaceable subassemblies:

```text
hardware/mechanical/
  assembly/             Full assemblies
  body/                 Waist, chest, neck, petticoat frame
  brackets/             Servo and actuator bracket primitives
  head/                 Skull, shell, gimbals, jaw, eyelids
  joints/               Standard bearing and hinge parts
  limbs/                Shoulder, arm, forearm, hand
```

## Joint groups

| Group | Initial DoF | Power source |
| --- | ---: | --- |
| Eyes | 4 | 3.3/5 V |
| Eyelids | 4 | 3.3/5 V |
| Brow | 2 | Bowden |
| Jaw | 1 | Bowden |
| Neck | 3 | Local |
| Torso waist | 1 | External |
| Gesture arm | 4 x 2 | External |

## Print rules

- Structural brackets: PETG or nylon; minimum four walls
- Decorative shell: PLA or resin; non-load-bearing
- Bearing seats: test coupon before printing
- Snap fits: avoid for parts that retain power or fall-risk loads
- Cable routing: 8 mm minimum bend radius for initial Bowden cable
- All structural parts require a load label and revision in metadata

## CAD TODO

The OpenSCAD files in `hardware/mechanical/` are parameter scaffolds. Do not use
them to fabricate structural parts until joint loads and factor of safety are
calculated and the CAD has been reviewed.

## Preliminary joint-load review

| Joint group | Load case | Target actuator class | Factor of safety | Status |
| --- | --- | --- | ---: | --- |
| Neck | 5 kg head, 100 mm offset, 1 g | ≤ 5 N m continuous; holding brake preferred | 3 dynamic / 5 static | Prototype only |
| Jaw | 1 N pinch load, 20 mm lever | ≤ 2 Nm stall, current-limited | 2 | Pinch test required |
| Eyes/eyelids | 0.1 N end load, 20 deg/s | Micro servo or stepper under current limit | 2 | 36 mm display isolation required |
| Shoulder | 2 kg arm, 400 mm radius | Actuator per measured test; 8 Nm nominal starting estimate | 3 | Not selected |
| Waist | 25 kg top-load, 100 mm offset | 40 Nm nominal starting estimate, holding brake | 4 | Not selected |

All torque values are provisional design gates, not approved ratings. A supplier
datasheet, 15-minute thermal test, stall-current test, and static toppling
calculation are required before any actuator is bought or structural bracket is
fabricated. Fasteners shall use metric property-class markings (minimum ISO 4762
class 8.8 for structural joints, unless a reviewed design documents a lower
rating). Printed brackets are not primary safety load paths until coupon tests
and a factor-of-safety analysis are passed.

The 160 mm head and 36 mm eye displays are display and proportion targets, not
validated anthropometric data. The 1:1 proportion shall be traced to a named,
versioned body-scan or mannequin standard before production CAD is released.
