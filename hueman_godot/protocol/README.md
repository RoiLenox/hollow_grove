# Hollow Grove Godot Protocol Fixtures

The request and response JSONL records are shared Rust/Godot contract fixtures
for gameplay protocol V1. They contain original Hollow Grove wire data only.

Godot submits the request envelope and displays the returned immutable events
and view. Rust remains authoritative over revision, identity, movement, party,
Bond, action, progression, House, Synthesis, and persistence state.

`scripts/runtime_client.gd` is transport-only. An intent listed in the schema
may still be unavailable until its authoritative Rust reducer is implemented;
the service reports that state with `CapabilityUnavailable`.

`MoveIntent` and `EnterRegionIntent` become available after Hueman is
established. The authoritative view includes the current typed map rows, tile
size, player position, and optional Boardwalk case projection; Godot does not
contain a duplicate collision map or case reducer.

The implemented Boardwalk wire intents are
`DiscloseFacultyObservationIntent`, `SupportBoardwalkOptionIntent`, and
`AskReturningGoonToDecideIntent`. The final event names the Returning Goon as
decision-maker and returns a typed outcome view with exact House-authority
actors, optional relationship kind/Bond/term, uncertainty, and refusal
protections. `SaveIntent` and `LoadIntent` use stable slot IDs and the service's
configured save root. Gameplay save schema V2 embeds the validated
institutional state loaded from the separate world root.

The Current Sea Stonebend case uses the same faculty intent plus
`SupportStonebendContinuityOptionIntent` and
`AskStonebendToDetermineContinuityIntent`. Its immutable view exposes the
outcome class, active and provisional Names, exact live Stonebend actor,
decision and Seal IDs, uncertainty, refusal paths, and the categorical fact
that no Title was granted. Godot never constructs those records.

Every overworld response also includes an immutable route view: stable route
ID, display Name, Straight/Round/Sea geometry, House endpoints, dominant verb,
purpose, and process. All ten route maps are accepted through
`EnterRegionIntent`, while the Rust runtime rejects transfers between routes
that share no House endpoint.
