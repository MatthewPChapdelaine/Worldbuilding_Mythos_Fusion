# Commissioning

## 1. Bench

1. Disconnect actuator power and verify e-stop opens actuator rail.
2. Measure zero positions and joint travel by hand.
3. Set software limits below mechanical limits.
4. Program PWM centers and 500-2500 us endpoints.
5. Enable one servo at low torque.
6. Add each actuator only after previous calibration passes.

## 2. Network

1. Boot broker and node in disconnection mode.
2. Verify node enters sleep and servo rail stays off.
3. Start daemon simulator.
4. Verify lease and status topics.
5. Verify packet loss and broker loss behavior.

## 3. Handoff

1. Bring source and target to matching neutral pose.
2. Ramp source output before disabling source.
3. Ramp target output to current visual pose.
4. Verify no simultaneous relay activation.
5. Re-run e-stop test.

## 4. Eye and face

1. Verify each display is mechanically isolated from the moving shell.
2. Test eyelids independently.
3. Limit jaw torque and verify no pinch remains.
4. Run a 15-minute thermal test.
5. Inspect cable routing after 100 cycles.
