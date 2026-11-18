#!/usr/bin/env python3

from pathlib import Path


required = [
    "README.md",
    "docs/00-safety.md",
    "docs/01-requirements.md",
    "docs/02-system-architecture.md",
    "config/nodes.yaml",
]


def main() -> None:
    missing = [path for path in required if not Path(path).is_file()]
    if missing:
        raise SystemExit("Missing required files: " + ", ".join(missing))
    print("Repository design structure is complete")


if __name__ == "__main__":
    main()
