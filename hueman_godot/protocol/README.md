# Hollow Grove Godot Protocol Fixtures

The request and response JSONL records are shared Rust/Godot contract fixtures
for gameplay protocol V1. They contain original Hollow Grove wire data only.

Godot submits the request envelope and displays the returned immutable events
and view. Rust remains authoritative over revision, identity, movement, party,
Bond, action, progression, House, Synthesis, and persistence state.

`scripts/runtime_client.gd` is transport-only. An intent listed in the schema
may still be unavailable until its authoritative Rust reducer is implemented;
the service reports that state with `CapabilityUnavailable`.

The live service synchronizes outside Hollow Grove as Roï and Rosaliña.
Godot submits `EnterHollowGroveIntent` and waits for the Rust-confirmed paired
transition before it establishes or resumes Hueman gameplay as Roi and
Roselina. `LeaveHollowGroveIntent` performs the reverse crossing. Both
operations share the request revision with gameplay commands, and transition
events arrive in the immutable `view.guide_transition_events` projection.
The retro overworld binds both directions to one context-aware `G` seam
command. Initial connection may enter once automatically; after a confirmed
leave, the client remains outside until `G` is pressed again.

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
configured save root. Gameplay save schema V4 embeds the validated
institutional state loaded from the separate world root and the explicit
Runtime Federation binding. The service save envelope additionally stores the
canonical dual-Guide session and unified operational revision, while legacy
bare gameplay archives remain readable. The typed
`AdvanceWingedGargoylePilotIntent` advances only the runtime-projected next
action in the ordered Recipe hunt; the Godot client cannot grant itself a
different stage or capability. The older generic `BeginSynthesisIntent`
remains an unavailable protocol reservation; it is not an alternate path into
the implemented pilot.

Every gameplay view also carries `kernel_audit`: a read-only report showing
whether the Federation binding and paired Roi/Roselina Guide oversight pass the
executable kernel invariants. The complete evidence-to-decision-save-replay
route is the `path.platinum.v1` Platinum Path, or “the Plath.”

The Boardwalk projection also carries the advisory-only `advisories` list for
Cleopatra (Gimp of the Underway), Hal (Pimp of the Boardwalk), and Clouseau
(PLEB-PLEB witness). Their presentation is centralized at the Central Junction
advisory desk under Roi and Roselina. Godot may display their finding,
uncertainty, refusal, and handoff text, but `world_state_committed` remains
false and the client still submits the existing support/decision intents to
Rust.

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
