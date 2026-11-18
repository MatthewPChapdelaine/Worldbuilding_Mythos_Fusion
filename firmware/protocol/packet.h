#pragma once

#include <stdint.h>

#define PROTOCOL_VERSION 1
#define JOINT_LIMIT 32
#define NODE_ID_MAX 32
#define LEASE_ID_MAX 32

typedef struct {
    uint16_t protocol_version;
    uint16_t node_id_length;
    char node_id[NODE_ID_MAX];
    uint32_t sequence;
    uint16_t lease_id_length;
    char lease_id[LEASE_ID_MAX];
    uint32_t expires_at_ms;
    uint8_t joint_count;
    int16_t normalized_milli[JOINT_LIMIT];
} pose_packet_t;
