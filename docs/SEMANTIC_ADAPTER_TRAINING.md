# Hollow Grove Shared Semantic Adapter Preparation

No model training is performed by this repository phase.

Training becomes appropriate only after the authoritative runtime has produced
enough confirmed and rejected `hollow-grove-semantic-witness-v1` records to
represent anchors, synonym proximity, contextual polysemy, overlaps,
translations, and constitutional rejection.

## One field, one adapter

The initial adapter architecture is:

```text
compatible base instruct model
        +
one Hollow Grove shared adapter
        +
retrieved constitutional canon
        +
Rust constitutional validation
```

There are no separate House personalities. Stonebend, Sandmanor, Glaüshouse,
and Flynt remain four immutable Points in one coordinate field.

The example configuration is
`ai/training/hollow-grove-adapter-v1.example.json`. It is disabled by default
and deliberately leaves the compatible base model for an operator to select.
The example Modelfile packages a compatible local result for Ollama; it does
not grant the model authority or perform training.

## Validate and split

First validate the deterministic export:

```text
hollow-grove dataset validate-semantic \
  ai/datasets/generated/semantic-witnesses-v1.jsonl
```

Then create a deterministic train/validation split with an explicit seed:

```text
hollow-grove dataset split-semantic \
  ai/datasets/generated/semantic-witnesses-v1.jsonl \
  ai/datasets/generated/semantic-witnesses-v1.train.jsonl \
  ai/datasets/generated/semantic-witnesses-v1.validation.jsonl \
  410041 \
  100
```

`validation-millis` is an integer from 0 through 1000. `100` assigns roughly
one tenth of a sufficiently varied dataset to validation. Assignment is a
stable hash of the explicit seed and witness reference. Output ordering is by
stable witness reference, and repeated runs over identical records are
byte-identical.

The split refuses duplicate witness references. It never shuffles records,
adds timestamps, exports hidden reasoning, changes canonical records, or runs a
trainer.

## Constitutional limits

Weights may learn vocabulary, contextual placement, overlap, evidence-aware
proposal structure, and rejection-aware behavior. Exact canon remains
retrieved. Coordinates remain fixed-point integers. Y remains zero. Model
output remains a proposal, and Festival remains the only confirmation boundary.

A custom coordinate head is explicitly deferred until the prompt-based
pipeline and witnessed datasets have been validated.
