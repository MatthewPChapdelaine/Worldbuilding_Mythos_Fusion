# Test Plan

Run protocol and safety tests on the workstation before hardware power is
enabled.

```bash
python3 tools/validate-repository.py
python3 -m pytest tests
```
