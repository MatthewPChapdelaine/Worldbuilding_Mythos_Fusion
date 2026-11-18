#!/usr/bin/env python3

import json
import time


def safe_pose() -> dict:
    return {
        "v": 1,
        "seq": 0,
        "lease": None,
        "expires_ms": 0,
        "joints": {},
    }


def main() -> None:
    while True:
        # TODO: Receive OSC, own lease, validate, and publish MQTT pose packets.
        print(json.dumps(safe_pose()))
        time.sleep(1)


if __name__ == "__main__":
    main()
