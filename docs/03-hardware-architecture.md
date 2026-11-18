# Hardware Architecture

## Shared head

- 2020 or 20x40 mm aluminum neck tube
- 3-axis optical eye gimbal
- Independent upper/lower eyelids
- Brow and jaw subassemblies
- Round IPS/OLED displays
- ESP32-S3 controller
- WS2812B- or RGBW-compatible lighting
- Bowden cables for eyelids, jaw, and brow

## Foyer node

- Standing 1:1 mannequin frame
- Weighted wheeled base with locking casters
- Neck, face, and two gesture arms
- Waist yaw limited to low-rate movement
- Passive first version; no autonomous leg actuation

## Kitchen node

- Upper torso or counter pivot
- Six-DoF arm assemblies for visual gesture only
- Food-zone exclusion and splash guard
- No grippers on the heating or cutting area

## Desk node

- Bust and shoulders
- Dense face/neck assembly
- Clamped weighted base
- No continuous high-torque actuators in the head

## Power model

| Rail | Purpose | Protection |
| --- | --- | --- |
| 12/24 V DC | Servo and motion bank | Fuse, contactor/relay, e-stop |
| 5 V DC | Displays and LEDs | 5 V fuse and bulk capacitance |
| 3.3 V DC | Controllers and sensors | Polarity protection and resettable fuse |

Do not share servo ground return with logic unless a power-architecture review
has verified return current and ground offsets.
