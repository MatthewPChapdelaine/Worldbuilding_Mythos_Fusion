import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "software" / "simulator"))

from simulate import validate_pose


def test_pose_acceptance():
    assert validate_pose({"head_yaw": 0.0})
    assert validate_pose({"head_yaw": -1.0, "jaw": 1.0})


def test_out_of_range_pose_is_rejected():
    assert not validate_pose({"head_yaw": 1.01})
