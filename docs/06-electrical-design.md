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

Wire and fuse sizes are not specified in this concept file. Calculate from:

1. Maximum continuous node current
2. Stall current of each actuator
3. Worst-case simultaneous duty cycle
4. Fuse I2t and cable ampacity
5. Connector temperature rating

## CAN/serial budget

The controller should support USB or UART diagnostics. The face controller should
be independently resettable. PWM commands may use a 12-bit range of 500-2500 us
after calibration.
