# Hollow Grove as a callable Ollama model

Hollow Grove can be installed under the local Ollama model name
`hollow-grove`. This gives it the same invocation shape as other Ollama
models while preserving the constitutional boundary:

```text
ollama run hollow-grove
        ↓
Aura semantic proposal
        ↓
Rust Rampart and Festival
        ↓
confirmed or rejected Current state
```

The Ollama model is not a second runtime. It cannot assign stable IDs, resolve
evidence, mutate the catalog, or confirm a transformation.

## Prerequisites

Install Ollama, start its local service, and install one base model:

```sh
ollama serve
ollama pull <base-model>
```

The base model is deliberately not hard-coded. Hollow Grove does not download
a model or start a service without an explicit operator action.

## Install the local model name

From the repository:

```sh
cargo run --bin hollow-grove -- model install-ollama <base-model>
```

After installing the Hollow Grove binaries, the shorter form is:

```sh
hollow-grove model install-ollama <base-model>
```

The default installed name is `hollow-grove`. An optional final argument
chooses a different local name:

```sh
hollow-grove model install-ollama <base-model> hollow-grove-dev
```

The installer generates a deterministic Ollama Modelfile, invokes
`ollama create`, and removes its temporary file. It does not pull the base
model and does not start Ollama.

The exact Modelfile can be inspected without changing Ollama:

```sh
hollow-grove model show-modelfile <base-model>
```

## Call it

Interactive or one-shot CLI:

```sh
ollama run hollow-grove --format json
ollama run hollow-grove --format json \
  "Restore the broken supporting frame."
```

The JSON constraint prevents a base model from decorating its proposal with
Markdown fences. The result is still untrusted and must pass Rust parsing and
Festival validation before it can affect canonical state.

Native local Ollama API:

```sh
curl http://localhost:11434/api/generate \
  -d '{
    "model": "hollow-grove",
    "prompt": "Restore the broken supporting frame.",
    "stream": false,
    "format": "json"
  }'
```

OpenAI-compatible local endpoint:

```sh
curl http://localhost:11434/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "hollow-grove",
    "messages": [
      {
        "role": "user",
        "content": "Restore the broken supporting frame."
      }
    ],
    "response_format": {
      "type": "json_object"
    }
  }'
```

These direct calls return proposal JSON. They do not confirm state.

## Call the authoritative path

Applications that require a real Hollow Grove state transition must send the
proposal through the existing Rust semantic parser, Rampart phases, and
Festival validation. A syntactically valid Ollama response means only
`Proposed`; it does not mean `Confirmed`.

Run the complete authoritative cycle with a Point already present in the
composition catalog:

```sh
hollow-grove rampart run \
  point.hueman.origin \
  structural-repair \
  "Restore the supporting frame."
```

The command performs:

```text
The Way Back
→ The Initiation through Ollama
→ The Gathering with retrieved Semantic Kernel V1 canon
→ The Festival
→ an atomically persisted confirmed or rejected witness
```

By default it uses the `hollow-grove` Ollama model and stores replayable
semantic provenance at:

```text
artifacts/semantic_rampart_state_v1.json
```

Set `HOLLOW_GROVE_MODEL` to select another installed Ollama interpreter. Use
`--state <path>` to select another state snapshot and `--json` for structured
output. Unknown Points are rejected; the command never creates an identity
from model output or merely because a name was supplied.

For deterministic offline validation, `--proposal-json <path>` replaces only
the Initiation interpreter response. The supplied proposal still passes through
the same Gathering, Festival, provenance, witness, and persistence path.

Local applications can use the same authoritative path over HTTP:

```sh
hollow-grove rampart serve \
  --listen 127.0.0.1:47820 \
  --state /home/warren/hollow_grove/artifacts/semantic_rampart_state_v1.json
```

Then submit a strict JSON request:

```sh
curl \
  -H 'Content-Type: application/json' \
  -d '{
    "point_ref": "point.hueman.origin",
    "scale_key": "structural-repair",
    "intent": "Restore the supporting frame.",
    "context_ref": null
  }' \
  http://127.0.0.1:47820/v1/rampart/cycles
```

The V1 listener is loopback-only. It serializes cycles through one authoritative
catalog and atomically persists the resulting provenance snapshot. See
`docs/RAMPART_HTTP_API_V1.md` for the complete contract, discovery routes,
status codes, limits, and trust boundary.

The local alias is a prompt-packaged model, not a newly trained foundation
model from the class of models published by Meta or Google. A future shared
Hollow Grove adapter can be attached beneath the same local model name after
witnessed training data is sufficient. Exact canon and confirmation remain in
Rust either way.
