#!/usr/bin/env python3

SAFE_POSE = {
    "head_yaw": 0.0,
    "head_pitch": 0.0,
    "jaw": 0.0,
    "left_brow": 0.0,
    "right_brow": 0.0,
}

JOINT_LIMITS = {
    "head_yaw": (-1.0, 1.0),
    "head_pitch": (-1.0, 1.0),
    "jaw": (0.0, 1.0),
    "left_brow": (-1.0, 1.0),
    "right_brow": (-1.0, 1.0),
}


def validate_pose(pose: dict) -> bool:
    if not isinstance(pose, dict):
        return False
    if not pose:
        return False
    try:
        for name, value in pose.items():
            if name not in JOINT_LIMITS:
                return False
            low, high = JOINT_LIMITS[name]
            if not low <= float(value) <= high:
                return False
        return True
    except (TypeError, ValueError):
        return False


if __name__ == "__main__":
    assert validate_pose(SAFE_POSE)
    assert not validate_pose({"head_yaw": 1.1})
    print("Pose validation checks passed")
