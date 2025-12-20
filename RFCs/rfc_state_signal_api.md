# RFC — State & Signal API

**Status:** Draft  
**Audience:** Core maintainers, contributors, early adopters  
**Scope:** Public API contract (conceptual + semantic)

---

## Purpose

Define the **State / Signal API** for Flurry that enables reactivity while preserving:

- Determinism
- Explicitness
- Strong typing
- Predictable performance

This RFC specifies **what the API guarantees and how it behaves**, not how it is implemented internally.

---

## Design Goals

The State / Signal API must:

- Enable UI updates when data changes
- Avoid implicit dependency tracking
- Avoid observers and watchers
- Make all reactive boundaries explicit
- Be deterministic and replayable
- Be compatible with GPU-first rendering

---

## Core Philosophy

Flurry does **not** use observer-based reactivity.

Instead:

> **State changes trigger explicit UI reevaluation, producing a new UI tree.**

The framework decides what actually updates.

Reactivity is a **controlled rebuild**, not ambient mutation.

---

## Terminology

- **State**: A mutable, typed container that holds application data
- **Signal**: A lightweight state primitive intended for UI-driven values
- **UI Reevaluation**: Re-running UI construction to produce a new UI tree

For v0, *state* and *signal* share the same conceptual model.

---

## Public API (Conceptual)

### Creating State

```rust
let count = state(0);
```

Properties:
- Strongly typed
- Explicitly created
- Lives outside the UI tree

---

### Reading State

```rust
let value = count.get();
```

Semantics:
- Returns the current value
- Reading state **does not** implicitly register dependencies
- Reading state is side-effect free

---

### Writing State

```rust
count.set(42);
```

Semantics:
- Replaces the stored value
- Marks the owning UI scope as dirty
- Schedules a UI reevaluation

There is no partial mutation or deep tracking.

---

## Reactivity Model

### What Happens on `set`

1. State value is updated
2. Framework schedules UI reevaluation
3. UI construction runs again
4. A new UI tree is produced
5. The framework diffs old vs new trees
6. Scene updates are applied

This process is deterministic and bounded.

---

### UI as a Pure Function of State

UI code must obey this rule:

> **Given the same state values, UI construction must produce the same UI tree.**

Violating this rule results in undefined behavior.

---

## Explicit Reactivity Boundaries

Flurry enforces explicit boundaries:

- Only state created via `state(...)` participates in reactivity
- Only calls to `set(...)` can trigger UI updates
- There is no automatic tracking of reads

If state is not explicitly wired into UI construction, it has no effect.

---

## No Implicit Dependency Tracking

Flurry intentionally avoids:

- Dependency graphs
- Reactive scopes
- Automatic invalidation based on reads

All reactivity is driven by **explicit writes**, not inferred reads.

This keeps behavior predictable and debuggable.

---

## Threading and Concurrency (v0 Assumptions)

- State updates are serialized by the framework
- UI reevaluation occurs on the UI scheduler
- No concurrent mutation of state during UI construction

These rules may be relaxed in future RFCs.

---

## Error Handling and Safety

- State access outside a valid UI context is a compile-time or runtime error (TBD)
- Reentrant state updates during UI construction are forbidden
- Infinite update loops are the developer’s responsibility to avoid

---

## What This API Intentionally Avoids

This RFC explicitly excludes:

- Computed state
- Derived signals
- Automatic memoization
- Async state primitives
- Time-based reactivity

These may be introduced via separate RFCs.

---

## Comparison to Other Models

- **Vue (classic):** observer-based reactivity
- **Vue Vapor Mode:** compiler-inferred dependencies
- **Flurry:** explicit state + deterministic UI rebuild

Flurry prioritizes correctness and explainability over convenience.

---

## Stability Policy

Once stabilized:

- Function names (`state`, `get`, `set`) are frozen
- Semantics are non-breaking
- Extensions require new RFCs

---

## One-Sentence Definition

> **Flurry state is an explicit, typed container whose mutations trigger deterministic UI reevaluation, never implicit observer updates.**

---

## Closing Notes

This State / Signal API is foundational.

It defines **how reactivity exists in Flurry** without compromising:

- Determinism
- GPU-first rendering
- Architectural clarity

All early implementations are disposable.

This contract is not.

