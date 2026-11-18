#!/usr/bin/env python3

import json
import struct
import time
from typing import Optional


PROTOCOL_VERSION = 1
MOTION_PACKET_MAX_AGE_MS = 500
JOINT_LIMITS = {
    "head_yaw": (-0.75, 0.75),
    "head_pitch": (-0.60, 0.60),
    "jaw": (0.0, 0.50),
}

OSC_JOINT_NAMES = {
    "/avatar/parameters/SpiritHeadYaw": "head_yaw",
    "/avatar/parameters/SpiritHeadPitch": "head_pitch",
    "/avatar/parameters/SpiritJaw": "jaw",
}


def safe_pose() -> dict:
    return {
        "v": PROTOCOL_VERSION,
        "seq": 0,
        "lease": None,
        "expires_ms": 0,
        "joints": {},
        "node": None,
    }


def validate_packet(packet: dict, lease_owner: Optional[str]) -> bool:
    if not isinstance(packet, dict):
        return False
    if packet.get("v") != PROTOCOL_VERSION:
        return False
    if not isinstance(packet.get("seq"), int) or packet["seq"] < 0:
        return False
    if not isinstance(packet.get("lease"), str) or packet["lease"] != lease_owner:
        return False
    if not isinstance(packet.get("node"), str) or not packet["node"]:
        return False
    ttl = packet.get("expires_ms")
    if not isinstance(ttl, (int, float)) or not 0 < ttl <= MOTION_PACKET_MAX_AGE_MS:
        return False
    joints = packet.get("joints")
    if not isinstance(joints, dict):
        return False
    for name, value in joints.items():
        if name not in JOINT_LIMITS:
            return False
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            return False
        low, high = JOINT_LIMITS[name]
        if not low <= float(value) <= high:
            return False
    return True


def build_pose_packet(node: str, lease: str, sequence: int, joints: dict) -> dict:
    if not all(name in JOINT_LIMITS for name in joints):
        raise ValueError("unknown joint")
    for name, value in joints.items():
        low, high = JOINT_LIMITS[name]
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            raise ValueError("invalid joint value")
        if not low <= float(value) <= high:
            raise ValueError("joint out of range")
    return {
        "v": PROTOCOL_VERSION,
        "seq": sequence,
        "lease": lease,
        "expires_ms": MOTION_PACKET_MAX_AGE_MS,
        "joints": joints,
        "node": node,
    }


def parse_osc_float_payload(address: bytes, payload: bytes) -> Optional[tuple[str, float]]:
    try:
        address_text = address.split(b"\0", 1)[0].decode("ascii")
    except UnicodeDecodeError:
        return None
    joint = OSC_JOINT_NAMES.get(address_text)
    if joint is None or len(payload) < 4:
        return None
    value = struct.unpack(">f", payload[:4])[0]
    if value != value:
        return None
    return joint, value


def main() -> None:
    while True:
        # Transport integration is intentionally isolated; no actuator is exposed
        # until a valid lease and packet pass validation.
        print(json.dumps(safe_pose()))
        time.sleep(1)


if __name__ == "__main__":
    main()
