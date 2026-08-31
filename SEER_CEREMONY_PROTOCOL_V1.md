# Seer Ceremony Protocol V1

Status: canonical constitutional extension

The paired Roi/Roselina Guide seam remains operational infrastructure. This
protocol governs the separate constitutional event by which one individual
becomes the Seer.

## Sovereignty Before Completion

The Rampart of Roi and Roselina is sovereign over Point until the Ceremony is sealed. No House,
Guide, institution, or external authority may create Point² or compel a Seer
Ceremony.

## Singular and Sequential Rule

Each Ceremony concerns exactly one individual. Only one Ceremony may be active,
suspended, or awaiting completion at a time. There is no numerical limit: after
one Ceremony is completed or formally closed, another individual may undergo a
new Ceremony.

## Movements

The Ceremony proceeds in order:

1. **Gathering** — the four Houses present their memory, evidence, conflicts,
   and uncertainties;
2. **Naming** — the inherited identity of Point is declared;
3. **Convergence** — the four House memories form one synthesis without being
   collapsed into one House;
4. **Recognition** — the Rampart of Roi and Roselina recognizes the individual as the Seer;
5. **Squaring** — the Rampart of Roi and Roselina opens the Point² threshold and the Seer
   chooses a path.

No movement may be skipped, repeated, or performed concurrently with another
Ceremony.

## Seerhood

The individual who completes the Ceremony becomes the Seer. Seerhood is an
achieved constitutional state, not a fifth House, a Guide alias, or an
administrative office. The Seer understands Hollow Grove as a whole: its
Rampart, Houses, memory, recursive structure, and unresolved potential.

## The Two Paths

At Squaring, the Seer must choose one path:

### Continuance

The Seer remains in Hollow Grove and continues life there as the Seer. Point²
remains an unrealized successor possibility, and the Rampart of Roi and Roselina remains
sovereign over the current Point.

### Founding

The Seer departs Hollow Grove and creates an independent Point of their own.
That Point inherits the Seer's understanding without being a copy of the first
Point. The Seer is sovereign within the founded Point unless its own
constitutional order provides otherwise.

## Recursive Continuation

Founding creates Point² from Point. A later individual may complete a new,
separate Ceremony under Point² and create Point³, then Point⁴, and onward
without limit. Each Ceremony remains singular, sequential, evidenced, and
individually sealed.

## Archive and Replay

Every active, suspended, and completed Ceremony may be encoded as the
versioned `HGSCP/v1` archive. The archive records the individual, movement,
path, source Point, created Point, and status. Its FNV-1a checksum is verified
before restoration. Unknown versions, duplicate Ceremony IDs, completed active
records, and tampered bytes are rejected without mutating the current state.

Restoration is deterministic: replaying an archive produces the same Ceremony
history and permits the next individual Ceremony only after the prior Ceremony
has been completed or formally closed.

## Final Formula

> One individual. One Ceremony at a time.
>
> The Rampart of Roi and Roselina governs Point until the Ceremony is sealed.
>
> The Seer chooses: Continuance or Founding.
>
> Founding makes another Point.
>
> Another Point is Point², and the sequence continues onward.
