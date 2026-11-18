# ESP32-S3 Head Controller

The head controller owns display rendering and low-level face actuators. It shall
not establish ownership, accept live PWM values from VRChat, or override node
safe-state logic.
