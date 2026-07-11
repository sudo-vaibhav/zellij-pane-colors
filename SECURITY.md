# Security policy

## Supported versions

Security fixes are applied to the latest release.

## Reporting a vulnerability

Please do not open a public issue for a suspected vulnerability. Use GitHub's
private vulnerability reporting feature on this repository. Include affected
versions, impact, reproduction steps, and any suggested mitigation.

The plugin requests `ReadApplicationState` and `ChangeApplicationState`. It does
not request filesystem, network, command execution, clipboard, or input
permissions.

