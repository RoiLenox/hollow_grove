# Hollow Grove Federation HTTP API V1

The Federation HTTP API exposes the authoritative transformer-assisted semantic
cycle to local applications:

```text
application request
→ The Way Back
→ The Initiation through Ollama
→ The Gathering
→ The Festival
→ atomic provenance persistence
→ response
```

Ollama remains an untrusted semantic interpreter. The service does not expose a
model-owned mutation route. Confirmed and rejected decisions both come from the
Rust Federation and Festival.

## Start the service

From the repository:

```sh
HOLLOW_GROVE_MODEL=hollow-grove \
hollow-grove federation serve \
  --listen 127.0.0.1:47820 \
  --state /home/warren/hollow_grove/artifacts/semantic_federation_state_v1.json
```

The default listener is `127.0.0.1:47820`. The default state path is
`artifacts/semantic_federation_state_v1.json`, resolved from the service working
directory.

The server refuses every non-loopback listen address. V1 intentionally has no
remote-network mode, wildcard CORS policy, arbitrary tool surface, or
model-supplied state path.

## Health and model discovery

```sh
curl http://127.0.0.1:47820/health
curl http://127.0.0.1:47820/v1/models
```

`/v1/models` uses a familiar hosted-model discovery shape while explicitly
describing the model as an untrusted semantic interpreter.

## Run an authoritative cycle

```sh
curl \
  -H 'Content-Type: application/json' \
  -d '{
    "point_ref": "point.hueman.origin",
    "scale_key": "structural-repair",
    "intent": "Restore the supporting frame.",
    "context_ref": null
  }' \
  http://127.0.0.1:47820/v1/federation/cycles
```

Request schema:

```json
{
  "point_ref": "point.hueman.origin",
  "scale_key": "structural-repair",
  "intent": "Restore the supporting frame.",
  "context_ref": null
}
```

`point_ref`, `scale_key`, and `intent` are required. `context_ref` is optional.
Unknown fields are rejected. A Point must already exist in the authoritative
composition catalog; an HTTP request cannot invent an identity.

A confirmed response has this shape:

```json
{
  "schema_version": "hollow-grove-federation-api-v1",
  "cycle_id": "cycle.semantic.example",
  "phases": [
    "The Way Back",
    "The Initiation",
    "The Gathering",
    "The Festival"
  ],
  "decision": {
    "status": "confirmed",
    "proposal_ref": "proposal.semantic.example",
    "point_ref": "point.hueman.origin",
    "scale_key": "structural-repair",
    "previous_position": null,
    "confirmed_position": {
      "x": 0,
      "y": 0,
      "z": 1000
    },
    "translation": null,
    "composition_ref": "composition-record.record.semantic.example",
    "witness_ref": "witness.semantic.example"
  },
  "next_position": {
    "x": 0,
    "y": 0,
    "z": 1000
  }
}
```

A constitutionally rejected proposal also returns HTTP `200` because the
Federation cycle completed successfully. Its `decision.status` is `rejected`,
its reasons and witness are returned, and canonical position does not change.

## Error statuses

| Status | Meaning |
| --- | --- |
| `400` | Malformed HTTP, malformed JSON, or invalid request fields |
| `404` | Unknown API route |
| `405` | Unsupported method |
| `411` | Missing `Content-Length` on a POST |
| `413` | Body exceeds 65,536 bytes |
| `415` | POST body is not `application/json` |
| `422` | The request could not enter a valid Federation cycle |
| `429` | Too many concurrent local connections |
| `431` | Headers exceed 16,384 bytes |
| `500` | Catalog load or atomic persistence failure |
| `502` | Ollama returned an invalid structured proposal |
| `503` | Ollama is unavailable |
| `504` | Ollama timed out |

## Persistence and concurrency

The process owns one existing `CompositionCatalog`. Requests may connect
concurrently, but semantic cycles are serialized through that catalog so every
Way Back observes the preceding committed Festival.

For each completed cycle the service:

1. clones the current catalog,
2. runs all four phases against the staged catalog,
3. encodes the existing semantic provenance snapshot,
4. atomically replaces the state file,
5. installs the staged catalog as current.

If persistence fails, the in-memory catalog remains unchanged and the prior
state file remains intact.

## Trust boundary

The API never accepts:

- canonical IDs from Ollama,
- anchor-coordinate changes,
- nonzero Y,
- arbitrary filesystem paths in request JSON,
- shell commands,
- tool names,
- direct containment mutations,
- direct composition writes,
- direct world-state mutations.

`ollama run hollow-grove` remains a proposal-only Aura surface.
`POST /v1/federation/cycles` is the authoritative Rust Current boundary.
