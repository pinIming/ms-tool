---
service: management
description: Exposes internal device-bind query and verification endpoints for service-to-service calls, allowing downstream services to verify device accessibility, look up bind records by SN or TID, and retrieve terminal payment parameters — all scoped to an operator.
nacos_name: management

keywords:
  - device
  - bind
  - SN
  - TID
  - MID
  - operator
  - payment-parameter
  - verify
  - runtime
  - acquirer

use_when:
  - When a downstream service needs to verify which devices an operator is allowed to access
  - When a service needs to look up a device bind record by SN or by TID
  - When a service needs terminal payment parameters (TID, MID, acquirer) for one or more devices
  - When a service needs current device runtime info (online status, signal strength)

not_responsible_for:
  - Creating, updating, or deleting device bind records (query-only)
  - Merchant or organization management
  - App/firmware deployment or push

depends_on:
  - authorization

domains:
  - name: DeviceBind
    summary: Internal device-bind query and verification endpoints scoped to an operator.
---

## Overview

The management service exposes internal APIs consumed by other microservices (e.g. fly-parameter, remote-command) to verify device accessibility, resolve bind records by SN or TID, and retrieve terminal payment parameters. All endpoints are operator-scoped and not intended for end-user clients.

---

## Domains

### DeviceBind
Handles device-bind queries and verification: look up bind records by SN or TID, verify device accessibility for an operator, fetch payment parameters, and retrieve device runtime info. NOT for creating or modifying bind records.
→ Read [`domains/device-bind.md`](domains/device-bind.md)

---

## Cross-Domain Relations ⭐

N/A — single domain service.

---

## References

- [`reference/openapi.yaml`](reference/openapi.yaml) — Full API spec. **Do NOT read whole.**
  Use each domain's API paths to grep for relevant sections.

