# Domain: DeviceBind

## Summary

Provides internal query and verification endpoints for device bind records, scoped to an operator. Use this domain to verify which devices are accessible to an operator, look up bind records by SN or TID list, fetch terminal payment parameters (TID/MID/acquirer), and retrieve live device runtime info. NOT for creating, updating, or deleting bind records.

## APIs

- ⭐ `POST /api/device-bind/operator/{operatorId}/device/verify` — **verify-device-access**
  Use when a service needs to check whether a list of device SNs is accessible to an operator, resolving the full sub-org tree first. NOT for TID-based identifiers — use `verify-batch` instead.

- ⭐ `POST /api/device-bind/operator/{operatorId}/device/verify/batch` — **batch-verify-device-access**
  Use when callers have TID-based (SN+TID) device identifiers and need to batch-verify operator access. Preferred over the SN-only verify endpoint when TIDs are available.

- `GET /api/device-bind/operator/{operatorId}/mrch/{mrchId}` — **list-devices-by-merchant**
  Use when fetching a paginated list of devices bound to a merchant under an operator. Note: `storeUuid` filtering is reserved and not yet implemented.

- `GET /api/device-bind/operator/{operatorId}/device/{devSn}/cmd/send/{isSendCmd}` — **get-device-runtime-info**
  Use when a service needs current device runtime state (online status, signal strength, battery, etc.). Set `isSendCmd=true` to trigger a remote command for fresh data; `false` returns cached state.

- `GET /api/device-bind/character/accessable` — **list-accessible-sns-by-character**
  Use when a downstream service needs to resolve the full list of device SNs accessible to a given principal (operator, agent, merchant, etc.) by character type and org scope.

> To get request/response schema for any API, derive the common path prefix from the
> API paths listed above (e.g., `/api/device-bind`), then:
> `grep -n "/api/device-bind" docs/contracts/management/reference/contract.yaml`
> Read only that line range (~20 lines of context).

**API Source:** `com.newland.modules.management.api.operator.DeviceBindServiceApi`

## Notes

- All endpoints require a valid `operatorId` — every query is scoped to an operator
- The `authorization` service must be reachable for the verification endpoints (`/verify`, `/verify/batch`, `/accessable`) — they resolve org hierarchies via Feign before checking device access
