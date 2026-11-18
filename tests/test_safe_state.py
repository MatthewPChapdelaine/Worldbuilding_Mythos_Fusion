import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "software" / "simulator"))

from simulate import validate_pose
sys.path.insert(0, str(ROOT / "software" / "spirit-daemon"))

from daemon import build_pose_packet, parse_osc_float_payload, validate_packet


def test_pose_acceptance():
    assert validate_pose({"head_yaw": 0.0})
    assert validate_pose({"head_yaw": -1.0, "jaw": 1.0})


def test_out_of_range_pose_is_rejected():
    assert not validate_pose({"head_yaw": 1.01})
    assert not validate_pose(None)
    assert not validate_pose({})


def test_packet_requires_matching_lease_and_node():
    packet = build_pose_packet(
        "kitchen-02",
        "lease-kitchen-02",
        123,
        {"head_yaw": 0.10, "head_pitch": -0.05},
    )
    assert validate_packet(packet, "lease-kitchen-02")

    wrong_owner = dict(packet)
    wrong_owner["lease"] = "lease-foyer-01"
    assert not validate_packet(wrong_owner, "lease-kitchen-02")


def test_packet_rejects_unknown_or_out_of_range_joint():
    packet = build_pose_packet("kitchen-02", "lease", 1, {"head_yaw": 0.0})
    packet["joints"]["unknown"] = 0.0
    assert not validate_packet(packet, "lease")

    packet = build_pose_packet("kitchen-02", "lease", 1, {"head_yaw": 0.0})
    packet["joints"]["jaw"] = 0.51
    assert not validate_packet(packet, "lease")


def test_osc_packet_maps_known_float_parameter():
    import struct

    result = parse_osc_float_payload(
        b"/avatar/parameters/SpiritHeadYaw\0",
        struct.pack(">f", 0.25),
    )
    assert result == ("head_yaw", 0.25)
