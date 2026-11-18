#include <Arduino.h>
#include "../protocol/packet.h"

static constexpr uint32_t MOTION_PACKET_MAX_AGE_MS = 500;
static constexpr uint32_t MOTION_RATE_MS = 1000 / 30;

static uint32_t last_packet_ms = 0;
static uint32_t last_motion_ms = 0;
static uint32_t last_sequence = 0;
static bool relay_enabled = false;

static void set_actuator_relay(bool enabled) {
    relay_enabled = enabled;
    digitalWrite(LED_BUILTIN, enabled ? HIGH : LOW);
}

static bool accept_packet(const pose_packet_t& packet, uint32_t now_ms) {
    if (packet.protocol_version != PROTOCOL_VERSION) return false;
    if (packet.sequence < last_sequence) return false;
    // expires_at_ms is an absolute Arduino monotonic-millisecond deadline.
    if (packet.expires_at_ms <= now_ms) return false;
    if (packet.joint_count > JOINT_LIMIT) return false;
    for (uint8_t i = 0; i < packet.joint_count; ++i) {
        if (packet.normalized_milli[i] < -1000 || packet.normalized_milli[i] > 1000) {
            return false;
        }
    }
    last_sequence = packet.sequence;
    last_packet_ms = now_ms;
    return true;
}

void setup() {
    Serial.begin(115200);
    pinMode(LED_BUILTIN, OUTPUT);
    set_actuator_relay(false);
}

void loop() {
    const uint32_t now_ms = millis();
    const bool packet_live =
        last_packet_ms != 0 && now_ms - last_packet_ms <= MOTION_PACKET_MAX_AGE_MS;

    if (!packet_live) {
        set_actuator_relay(false);
    }

    if (packet_live && now_ms - last_motion_ms >= MOTION_RATE_MS) {
        last_motion_ms = now_ms;
        // PWM calibration and transport integration are separate transport/CAD tasks.
        // This loop provides the required lease/watchdog gate.
    }
}
