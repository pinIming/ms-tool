# ms-tool — Product Requirements Document

**Version:** 1.0.0
**Status:** Draft
**Owner:** skill-app-parameter-configuration
**Date:** 2026-04-15

---

## Purpose

`ms-tool` is a Rust CLI binary that provides structured, on-demand access to microservice
contract files. It is called by Claude Code skills (primarily the `microservice` skill) to
retrieve overview, domain, and API spec content without reading entire files into context.

---

## Background

The `.microservice/` directory holds contract files for every registered microservice:

```
.microservice/
  <service-name>/
    <service-name>-overview.md   ← service summary + domain list
    domains/
      <domain-name>.md           ← API list with intent descriptions
    reference/
      openapi.yaml               ← full OpenAPI 3.x spec
```

Skills need targeted access to these files. Reading full openapi.yaml files into context
is expensive and noisy. `ms-tool` extracts only what is needed.

---

## Commands

### `ms-tool overview <service-name>`

**Description:** Prints the full content of the service overview file.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `service-name` | string | yes | Directory name under `.microservice/` (e.g. `management`) |

**Resolved path:** `$PWD/.microservice/<service-name>/<service-name>-overview.md`

**Output:** Full file content to stdout.

**Errors:**
- Exit 1 + `error: service '<service-name>' not found at .microservice/<service-name>/` if directory is missing
- Exit 1 + `error: overview file not found: .microservice/<service-name>/<service-name>-overview.md` if file is missing

**Example:**
```bash
ms-tool overview management
# → prints full content of .microservice/management/management-overview.md
```

---

### `ms-tool domain <domain-name> --service <service-name>`

**Description:** Prints the full content of a domain contract file.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `domain-name` | string | yes | File stem under `domains/` (e.g. `device-bind`) |
| `--service` | string | yes | Service directory name (e.g. `management`) |

**Resolved path:** `$PWD/.microservice/<service-name>/domains/<domain-name>.md`

**Output:** Full file content to stdout.

**Errors:**
- Exit 1 + `error: service '<service-name>' not found` if service directory is missing
- Exit 1 + `error: domain '<domain-name>' not found in service '<service-name>'` if domain file is missing

**Example:**
```bash
ms-tool domain device-bind --service management
# → prints full content of .microservice/management/domains/device-bind.md
```

---

### `ms-tool api "<api-path>" --service <service-name>`

**Description:** Extracts the full path entry for a given API path from the service's openapi.yaml.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `api-path` | string | yes | API path as it appears in openapi.yaml (e.g. `/api/device-bind/operator/{operatorId}/device/verify`) |
| `--service` | string | yes | Service directory name (e.g. `management`) |

**Resolved path:** `$PWD/.microservice/<service-name>/reference/openapi.yaml`

**Matching rule:** Locate the `paths:` section in the YAML, find the key that exactly matches
`<api-path>`, and return the complete path object including all HTTP methods, parameters,
requestBody, and responses. Do not truncate.

**Output:** The matched YAML block to stdout, in valid YAML format.

**Errors:**
- Exit 1 + `error: openapi.yaml not found for service '<service-name>'` if file is missing
- Exit 1 + `error: path '<api-path>' not found in openapi.yaml for service '<service-name>'` if path key is absent

**Example:**
```bash
ms-tool api "/api/device-bind/operator/{operatorId}/device/verify" --service management
# → prints the full YAML block for that path from openapi.yaml
```

---

## Path Resolution

- All paths are resolved relative to `$PWD` at invocation time (the project root).
- The tool does NOT walk up the directory tree to find `.microservice/`.
- Invoke `ms-tool` from the project root (same directory that contains `.microservice/`).

---

## Output Format

- All output goes to **stdout only**.
- No extra headers, banners, or decorators are added.
- Error messages go to **stderr**.
- Exit code `0` on success, `1` on any error.

---

## Recommended Rust Crates

| Crate | Purpose |
|-------|---------|
| `clap` (v4, derive feature) | CLI argument parsing and subcommand dispatch |
| `serde_yaml` or `yaml-rust2` | YAML parsing for the `api` subcommand |
| `anyhow` | Error propagation |
| `std::fs` | File reading (no async needed) |

---

## Non-Goals

- This tool does NOT write or modify any files.
- This tool does NOT sync or clone the `.microservice/` repository (that is `microservice-context.sh`'s job).
- This tool does NOT validate OpenAPI spec correctness.
- This tool does NOT support fuzzy/partial path matching — paths must match exactly.
- This tool does NOT need a configuration file — all inputs are CLI arguments.

---

## Installation

Build and install to a location on `$PATH` so skills can call it without a full path:

```bash
cargo build --release
cp target/release/ms-tool ~/.local/bin/ms-tool   # or any directory on $PATH
```

Skills detect absence of the tool and fall back to direct file reads (documented in
`.claude/skills/microservice/SKILL.md`).
