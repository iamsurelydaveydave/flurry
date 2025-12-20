# RFC Note — Reactivity Model Comparison (Flurry vs Vue Vapor Mode)

**Status:** Draft  
**Audience:** Core maintainers, contributors, early adopters  
**Scope:** Conceptual clarification (non-binding, explanatory)

---

## Purpose

This RFC note exists to **lock the conceptual comparison** between **Flurry’s reactivity model** and **Vue.js Vapor Mode**, in order to:

- Prevent confusion during onboarding
- Avoid incorrect assumptions about observers, watchers, or implicit reactivity
- Establish clear mental boundaries for future API and engine design

This document is **explanatory**, not an implementation specification.

---

## High-Level Positioning

Flurry’s reactivity model is **conceptually closest to Vue.js Vapor Mode**, but is:

- More explicit
- More deterministic
- More restrictive by design

The similarity is intentional at a *philosophical* level, not at an implementation level.

---

## Shared Philosophy

Flurry and Vue Vapor Mode share the following high-level ideas:

1. **Compile-time driven UI**  
   UI structure is known ahead of time; there is no runtime template parsing.

2. **Minimal runtime overhead**  
   Avoid heavy runtime abstractions such as Virtual DOM diffing or observer graphs.

3. **State-driven updates**  
   UI output changes only when underlying state changes.

4. **Declarative authoring, optimized execution**  
   Developers write declarative UI; the system performs targeted updates internally.

These similarities make the comparison valid and useful for intuition.

---

## Key Differences (Non-Negotiable)

Despite the surface similarity, Flurry intentionally diverges in several critical areas.

---

### 1. No Implicit Dependency Tracking

**Vue Vapor Mode:**
- Uses compiler-inferred reactive dependencies
- Tracks which state is read during render

**Flurry:**
- No implicit dependency tracking
- No automatic reactivity based on reads
- Reactivity exists only through **explicit state primitives**

Nothing becomes reactive unless the developer explicitly wires it.

---

### 2. Explicit UI Tree Rebuild Model

**Vue Vapor Mode:**
- Performs targeted DOM updates inferred from reactive scopes

**Flurry:**
- State changes trigger a **controlled UI tree reevaluation**
- A new UI tree is produced
- The framework diffs trees and updates the scene

Renderers never mutate UI nodes directly.

---

### 3. Determinism as a Hard Guarantee

Flurry guarantees:

- Same state → same UI tree
- Same UI tree → same layout
- Same layout → same scene

Vue Vapor Mode aims for predictability, but does not enforce determinism as a formal contract.

In Flurry, determinism is **non-negotiable**.

---

### 4. Output Target

**Vue Vapor Mode:**
- Direct DOM operations

**Flurry:**
- Scene graph generation
- GPU-first rendering pipeline
- CPU rendering as a strict fallback

Reactivity ultimately updates a **renderable scene**, not DOM nodes.

---

## What “Reactive” Means in Flurry

In Flurry:

- UI is a pure function of state
- State changes cause UI reevaluation
- Reevaluation produces a new UI tree
- The framework decides what actually updates

There are:
- No watchers
- No observers
- No magic propagation
- No runtime mutation of UI nodes

This is **controlled reactivity**, not ambient reactivity.

---

## Mental Mapping (Recommended)

For onboarding and discussion, the following mental mapping is accurate:

- Vue (classic) → Observer-based reactivity + Virtual DOM
- Vue Vapor Mode → Compile-time reactivity + direct DOM ops
- **Flurry → Explicit state + deterministic UI rebuild + scene diffing**

Flurry is philosophically closer to **Vapor Mode**, but architecturally closer to **Flutter-style retained UI**, adapted to Rust and GPU-first rendering.

---

## One-Sentence Canonical Definition

> **Flurry’s reactivity is similar in spirit to Vue Vapor Mode, but relies on explicit state boundaries and deterministic UI rebuilding rather than implicit dependency tracking.**

This sentence is approved for documentation, README usage, and contributor communication.

---

## Non-Goals of This RFC Note

This document does **not** define:

- The concrete state or signal API
- Performance characteristics
- Scheduling or batching semantics
- Multithreading behavior

Those concerns require separate RFCs.

---

## Closing Notes

This comparison exists to clarify intent, not to borrow implementation details.

Flurry deliberately trades convenience for:
- Predictability
- Explainability
- Architectural integrity

Any future reactivity-related RFCs must remain consistent with this comparison.