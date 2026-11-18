# VRChat UdonSharp Bridge

UdonSharp shall emit:

- Head yaw/pitch
- Eye look and vergence
- Blink, brow, jaw, and cheek blendshapes
- Animation event flags

It shall not emit physical PWM values, MQTT addresses, or node serial numbers.

VRChat is an optional input channel. The physical safety case is independent:
network loss, broker loss, or lease expiry must put every node in sleep state
without a VRChat client running.
