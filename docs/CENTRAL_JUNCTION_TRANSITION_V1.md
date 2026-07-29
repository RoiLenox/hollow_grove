# Central Junction Guide Transition V1

Status: canonical paired form-transition contract

## Seam

Central Junction remains `[0,0,0]`, outside the House enum, and between the
four constitutional poles. It is also the only V1 seam for the principal
Guide transition:

```text
Roï       ↔ Roi
Rosaliña  ↔ Roselina
```

This is not House-coordinate translation or ordinary semantic movement. It is
a dedicated form transition of continuous identity across layers.

## Directions

Entry:

```text
guide.roi:      Seer / OutsideHollowGrove → HollowGroveGuide / HollowGrove
guide.roselina: Seer / OutsideHollowGrove → HollowGroveGuide / HollowGrove
```

Exit:

```text
guide.roi:      HollowGroveGuide / HollowGrove → Seer / OutsideHollowGrove
guide.roselina: HollowGroveGuide / HollowGrove → Seer / OutsideHollowGrove
```

## Stages

```text
Approaching
AtOrigin
FormChanging
Completed
Rejected
```

Approaching, AtOrigin, and FormChanging are transaction-internal. They may
temporarily contain asymmetric staged values, but those values are never an
externally canonical completed session.

## Required Evidence

Every proposed individual transition records:

- transition ID;
- stable Guide identity;
- source form, layer, and public name;
- transition direction;
- destination form, layer, and public name;
- continuous session;
- source witness where present;
- continuity proof;
- embodiment proof.

Every confirmed paired witness additionally records:

- both individual transitions;
- all junction stages;
- Federation decision;
- resulting session state;
- stable identity before and after.

## Atomicity

V1 confirms the principal Surface↔Hueman crossing only when both individual
transitions are valid. Validation and staging operate on a candidate session.
The authoritative session is replaced only after both transitions and proofs
pass.

Rejection changes neither Guide form, neither layer, nor the current witness.
A partial completed crossing is constitutionally invalid.

## Authority

Inside Hollow Grove, Roi and Roselina are equal rulers of Central Junction.
Outside Hollow Grove, Roï and Rosaliña retain that same authority through their
Seer forms. The vertical seam is not a hierarchy and creates no external
supervisor.
