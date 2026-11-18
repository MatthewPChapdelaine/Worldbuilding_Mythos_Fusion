# Bill of Materials

Quantities marked N are adjusted per physical node.

## Computing

| Item | Quantity | Notes |
| --- | --- | --- |
| ESP32-S3 dev/controller board | N | USB-C and 3.3 V GPIO; 5 V tolerant not assumed |
| PCA9685 or equivalent PWM board | N | Isolated servo rail |
| Round IPS/OLED display | 2/head | 240x240 or smaller initial prototype |
| MOSFET/relay actuator bank | 1/node | Normally-off, hardware e-stop compatible |
| Managed local MQTT host | 1 | Minimum 2 GB RAM for initial build |
| Presence and door sensors | 1-2/node | Normally-safe wiring |

## Mechanical

| Item | Quantity | Notes |
| --- | --- | --- |
| 2020/20x40 aluminum extrusion | As frame | Structural design pending load calculation |
| PLA/PETG print filament | TBD | PETG-CF preferred for brackets only if supplier properties verified |
| 20 kg cm (2 Nm) current-limited micro servo | Bench only | Replaced if jaw/neck test loads exceed rated output |
| High-torque shoulder/waist actuator | N | Select only after the mechanical joint-load table is measured and approved |
| 2-4 mm Bowden cable | Head | Verified flex/strain test |
| Ball bearings and bushings | TBD | No printed load-bearing bearing in first prototype |
| Locking casters | 4/base | One brake path for all or independent dual brakes |

## Power

| Item | Notes |
| --- | --- |
| Protected 25.2 V LiFePO4 battery pack | Minimum 100 Wh usable plus 25% reserve; final 15-minute duty-cycle calculation required |
| BMS | 20 A continuous minimum for the example rail, exact rating tied to measured stall/peak current |
| DC/DC converters | 24 V -> 5 V at 5 A and 24 V -> 3.3 V at 2 A minimum; final ratings from thermal test |
| Main fuse | Sized below 12 AWG ampacity and measured cable run ampacity |
| E-stop and contactor | Remove battery-side actuator power |

## Clothing and dress

Use flame-resistant fabric for the skirt and apron, keep heat sources enclosed,
and create a removable access panel over the power bay. Petticoat structure shall
not carry a person's weight or obstruct node ventilation.
