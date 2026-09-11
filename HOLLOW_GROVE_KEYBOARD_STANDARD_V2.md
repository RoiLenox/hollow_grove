# Hollow Grove Keyboard Standard V2

Status: active operational standard

This standard refines the V1 implementation without changing the governing
`Key = WHAT · Modifier = HOW` constitution. H, J, K, and L remain movement and
layout tools; Space remains the Glaüshouse room/layout landmark.

## Shared quarter-height contract

Space and H/J/K/L operate on one canonical height ladder:

```text
25% → 50% → 75% → 100%
```

| Binding | Operation |
|---|---|
| `Super+Ctrl+J` | Cycle the ladder, anchored downward |
| `Super+Ctrl+K` | Cycle the ladder, anchored upward |
| `Super+Ctrl+Shift+Space` | Same upward cycle as `Super+Ctrl+K` |
| `Super+Alt+Shift+Space` | Same downward cycle as `Super+Ctrl+J` |
| `Super+Alt+Shift+H` | Select 25% directly, anchored downward |
| `Super+Alt+Shift+J` | Select 50% directly, anchored downward |
| `Super+Alt+Shift+K` | Select 75% directly, anchored downward |
| `Super+Alt+Shift+L` | Select 100% directly, anchored downward |
| `Super+Ctrl+Alt+Shift+Space` | Toggle the 25% bookmark and 75% focus stages |

The cycle and direct-selection routes must use these same four stages in the
personal Niri profile and the General Edition portable profile. Machine-local
paths may differ; behavior may not.

## Verification

Validate both Niri profiles and run
`packaging/hollow-grove-general/verify-keyboard-grove.sh`. The verifier rejects
any future drift between the Space cycle and H/J/K/L direct selections.
