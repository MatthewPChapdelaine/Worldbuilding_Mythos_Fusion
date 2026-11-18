# Electrical Design

## Controller topology

```text
Battery -> main fuse -> e-stop contactor -> actuator fuse -> servo rail
                                        \-> DC/DC -> logic fuse -> ESP32
                                        \-> DC/DC -> display/LED rail
```

## Grounding

- Battery negative is the reference ground.
- Use a star at the battery side.
- Route high-current servo returns separately from logic signal cables.
- Optically isolate or use a low-threshold MOSFET for relay control.
- Add TVS protection to external 5 V and 12/24 V power connectors.

## Wire and fuse sizing

Use a 25.2 V nominal LiFePO4 actuator rail (24 V BMS/cutoff compatible) and
provide separate 5 V display and 3.3 V logic converters. Final wire sizes and
fuses require measured node current; the following is a procurement/budget
assumption and shall not authorize energization:

| Assumption | Budget |
| --- | --- |
| Worst-case duty | 15 minutes at 40% actuator current, 60% of rated stall |
| Motion rail average | 8 A continuous |
| Motion rail peak | 20 A, 1 second maximum |
| 5 V display/LED rail | 5 A continuous |
| 3.3 V logic | 2 A continuous |

The 15-minute energy budget is 25.2 V x 8 A x 0.25 h = 50.4 Wh, plus 5 V x
5 A x 0.25 h = 6.25 Wh, plus 3.3 V x 2 A x 0.25 h = 1.65 Wh. Size the pack for
at least 100 Wh usable capacity plus 25% reserve, and verify low-voltage cutoff
and thermal limits; a 25.2 V 12 Ah (302 Wh nominal) protected LiFePO4 pack is
an example, not a release selection.

Starting point after ampacity review: 12 AWG from battery to contactor/fuse,
12 AWG actuator rail, 16 AWG 5 V rail, and 18-20 AWG logic rail, with lengths
and ambient temperature included in the calculation. Final fuse ratings shall
be below measured cable ampacity and above normal inrush but below conductor
and actuator-bank fault limits. Every converter shall have a datasheet for input
range, output current, thermal derating, and short-circuit behavior.

The first prototype shall use no 24 V distribution in the interaction zone.

## CAN/serial budget

The controller should support USB or UART diagnostics. The face controller should
be independently resettable. PWM commands may use a 12-bit range of 500-2500 us
after calibration.
