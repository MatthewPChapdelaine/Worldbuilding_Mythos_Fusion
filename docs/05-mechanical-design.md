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
