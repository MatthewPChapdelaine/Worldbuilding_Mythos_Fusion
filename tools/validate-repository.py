#!/usr/bin/env python3

from pathlib import Path


required = [
    "README.md",
    "docs/00-safety.md",
    "docs/01-requirements.md",
    "docs/02-system-architecture.md",
    "config/nodes.yaml",
    "firmware/protocol/packet.h",
    "tests/test_safe_state.py",
]


def main() -> None:
    missing = [path for path in required if not Path(path).is_file()]
    if missing:
        raise SystemExit("Missing required files: " + ", ".join(missing))
    packet_header = Path("firmware/protocol/packet.h").read_text()
    required_fields = (
        "protocol_version",
        "node_id",
        "sequence",
        "lease_id",
        "expires_at_ms",
        "normalized_milli",
    )
    absent = [field for field in required_fields if field not in packet_header]
    if absent:
        raise SystemExit("Packet header missing fields: " + ", ".join(absent))
    print("Repository design structure is complete")


if __name__ == "__main__":
    main()
