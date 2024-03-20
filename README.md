# cpu-cli-controller

Control linux CPUs via the CLI.

Cores can be:
- activated
- deactivated
- shown

The cores are detected by finding the directories of the form '/sys/devices/system/cpu/cpu<n>', where <n> is a positive integer. To activate (resp. deactivate) them, it sets '/sys/devices/system/cpu/cpu<n>/online' to `1` (resp. `0`). The cpu0 cannot be deactivated.
