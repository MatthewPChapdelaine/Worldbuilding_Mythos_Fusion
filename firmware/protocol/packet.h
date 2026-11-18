#pragma once

#include <stdint.h>

#define PROTOCOL_VERSION 1
#define JOINT_LIMIT 100

typedef struct {
    uint32_t sequence;
    uint32_t expires_at_ms;
    uint8_t joint_count;
    int16_t normalized_milli[JOINT_LIMIT];
} pose_packet_t;
