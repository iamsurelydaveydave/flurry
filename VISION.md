## 1. What Flurry Is

Flurry is a **Rust-native UI platform**, not just a UI library.

It consists of two tightly coupled parts:

* **Flurry Framework** – a Rust UI framework used to author applications
* **Flurry Runtime** – a browser-like runtime that loads, isolates, and renders Flurry apps

Flurry treats UI as a **compiled systems problem**, not a scripting problem.

UI, types, layout, rendering, and execution live in a single coherent system.

---

## 2. What Flurry Is NOT

Flurry is intentionally **not**:

* A web framework
* A replacement for HTML, CSS, or JavaScript
* A browser engine
* A JavaScript runtime
* A thin wrapper over WebView

Flurry does not inherit browser semantics, DOM behavior, or JS execution models.

---

## 3. Core Vision

Flurry exists to provide a **deterministic, GPU-first UI runtime** where:

* Layout is predictable
* Performance is stable
* Behavior is consistent across machines
* Types are real and enforced by the compiler

Flurry prioritizes **correctness and control** over convenience and legacy compatibility.

---

## 4. Non-Negotiable Design Values

These values must never be violated.

### 4.1 Deterministic Layout

* Layout results must be identical given the same input
* No hidden measurement passes
* No environment-dependent behavior
* GPU and CPU paths must compute identical geometry

Layout is computed once and never reinterpreted by the renderer.

### 4.2 Predictable Performance

* No unbounded work per frame
* No implicit layout thrashing
* No hidden allocations during render
* Performance characteristics must be understandable from the code

Performance is a property of design, not an afterthought.

### 4.3 GPU-First Rendering

* GPU is the primary execution path
* WGPU is the reference backend
* CPU rendering exists only as a fallback
* CPU fallback must match layout and interaction semantics

The GPU path defines correctness.

### 4.4 Strong, Real Types

* No erased runtime types
* No stringly-typed attributes
* UI state and events are typed
* Compile-time errors are preferred over runtime checks

Types are part of the runtime contract.

---

## 5. Mental Model

Flurry follows a **retained-mode UI model**:

1. Developer declares UI
2. Framework produces a UI tree
3. Layout engine computes geometry
4. Scene is rendered by a backend

Renderers do not:

* Recompute layout
* Run business logic
* Mutate application state

This separation is absolute.

---

## 6. Platform Boundary

Flurry applications **never run directly on the OS**.

All applications run inside the **Flurry Runtime**, which:

* Sandboxes applications
* Controls access to GPU, input, filesystem, and network
* Enforces explicit permission policies
* Prevents app-to-app interference

This is a hard security boundary, not a convenience layer.

---

## 7. Scope Discipline (Early Stage)

Flurry will deliberately avoid:

* Chasing feature parity with browsers
* Supporting legacy web behavior
* Optimizing prematurely
* Overengineering the engine before contracts are locked

Early work focuses on:

* Vision
* Naming
* API shape
* Contracts
* Deterministic behavior

All early implementations are disposable.

---

## 8. Success Definition

Flurry succeeds if:

* UI behavior is boringly predictable
* Performance characteristics are explainable
* Developers trust the system
* The architecture scales without conceptual debt

Flurry is built for the long term.
