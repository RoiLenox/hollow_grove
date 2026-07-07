# Client v0.4.1

Date: 2026-07-06

## Conclusion

The first real client should be the Script/Automation Client.

Reason:

It can consume `KernelPass` without changing the kernel, without adding Driver, history, replay, continuing state, routing, or semantic overreach.

## Deferral

- Desktop/OS client is deferred.
- Current Synthesis game/world client is deferred.
- AI workflow assistant is compatible but deferred.
- Script/automation client is the first real client.

## Boundary

Flow remains:

```text
Kernel
↓
KernelPass
↓
Client
```

Client must stay outside the kernel.
