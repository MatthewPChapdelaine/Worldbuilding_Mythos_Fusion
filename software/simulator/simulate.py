#!/usr/bin/env python3

SAFE_POSE = {
    "head_yaw": 0.0,
    "head_pitch": 0.0,
    "jaw": 0.0,
    "left_brow": 0.0,
    "right_brow": 0.0,
}


def validate_pose(pose: dict) -> bool:
    return all(-1 <= float(value) <= 1 for value in pose.values())


if __name__ == "__main__":
    assert validate_pose(SAFE_POSE)
    assert not validate_pose({"head_yaw": 1.1})
    print("Pose validation checks passed")
