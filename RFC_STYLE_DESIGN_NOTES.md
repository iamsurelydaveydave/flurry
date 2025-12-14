# Flurry RFC-Style Design Notes

---

## RFC-0000: RFC Process and Scope

### Status

Draft

### Purpose

Define how RFCs are written, evaluated, and adopted in the Flurry project.

### Motivation

Flurry prioritizes architectural correctness and long-term coherence. RFCs exist to capture *decisions*, *trade-offs*, and *non-goals* before code is written. Code may be deleted. Decisions must persist.

### Principles

* RFCs describe **why**, not implementation details
* RFCs define **contracts**, not optimizations
* RFCs are backend-agnostic unless explicitly stated
* RFCs may be revised, but not casually

### Non-Goals

* Not a design-by-committee process
* Not a replacement for code reviews

---

## RFC-0001: Rendering Architecture

### Status

Draft

### Summary

Define the high-level rendering pipeline for Flurry, including strict separation between layout, scene construction, and rendering backends.

### Motivation

UI rendering must be deterministic, predictable, and GPU-first. Rendering architecture must prevent hidden layout work, duplicated computation, and backend-specific behavior divergence.

### Architecture Overview

```
Declarative UI
      ↓
UI Tree
      ↓
Layout Engine
      ↓
Scene / Display List
      ↓
Renderer Backend
   ├─ WGPU (Primary)
   └─ TinySkia (Fallback)
```

### Key Decisions

* Layout is computed **once** per update cycle
* Renderers consume immutable scene data
* Renderers do not execute business logic
* GPU backend defines correctness

### Invariants

* Identical geometry across GPU and CPU backends
* No renderer may trigger layout recomputation
* Scene construction is backend-agnostic

### Trade-Offs

* Slight memory overhead for scene storage
* Less flexibility for renderer-specific optimizations

### Non-Goals

* Pixel-perfect parity across backends
* Supporting multiple layout engines

---

## RFC-0002: Layout Model

### Status

Draft

### Summary

Define Flurry’s deterministic layout system based on explicit row/column composition and constraint-driven sizing.

### Motivation

Layout unpredictability is a primary source of UI bugs and performance issues. Flurry enforces a small, explicit layout vocabulary to ensure predictability.

### Layout Model

* Retained-mode layout
* Parent-driven constraints
* Single-pass layout resolution

### Core Primitives

* `row`: horizontal flow
* `column`: vertical flow
* `stack`: z-axis overlay
* `spacer`: flexible or fixed space

### Sizing Rules

* Fixed sizes are absolute
* `fill()` consumes remaining space
* `auto()` sizes to content

### Invariants

* Same input yields same geometry
* No implicit measurement passes
* No environment-dependent sizing

### Trade-Offs

* Reduced flexibility compared to CSS
* More explicit developer responsibility

### Non-Goals

* CSS compatibility
* Constraint solvers or multi-pass layout

---

## RFC-0003: Element and Attribute Model

### Status

Draft

### Summary

Define how UI elements, attributes, and events are represented as typed, compile-time constructs.

### Motivation

Stringly-typed UI APIs lead to runtime errors and tooling-only safety. Flurry treats UI as a typed data structure.

### Element Model

* Elements are typed nodes
* Element identity is explicit (`key`)
* Elements are immutable after construction

### Attribute Model

* Attributes are strongly typed
* No string-based attribute lookup
* Global attributes apply uniformly

### Event Model

* Events are explicit function bindings
* Event signatures are element-specific
* No dynamic event dispatch

### Invariants

* Invalid attributes fail at compile time
* Events cannot mutate layout directly

### Trade-Offs

* More verbose API surface
* Less dynamic behavior

### Non-Goals

* Runtime reflection
* Dynamic attribute injection

---

## RFC-0004: UI Tree Lifecycle

### Status

Draft

### Summary

Define the lifecycle stages of a Flurry UI tree from declaration to rendering.

### Lifecycle Stages

1. UI declaration (macro expansion)
2. UI tree construction
3. Layout computation
4. Scene generation
5. Rendering

### Invariants

* Each stage is pure and isolated
* Later stages cannot mutate earlier outputs

### Trade-Offs

* Reduced runtime flexibility
* Clearer mental model

### Non-Goals

* Hot-swapping layout engines
* Runtime UI mutation APIs

---

## RFC-0005: CPU Fallback Policy

### Status

Draft

### Summary

Define the role and limitations of CPU rendering in Flurry.

### Motivation

GPU rendering is the primary path. CPU rendering exists only for safety, testing, and headless execution.

### Policy

* CPU renderer must consume the same scene data
* CPU renderer must match layout and interaction semantics
* Performance degradation is acceptable

### Invariants

* Layout parity with GPU
* No feature-only-on-CPU behavior

### Non-Goals

* CPU performance parity
* Advanced CPU-specific effects

---

## RFC-0006: Platform Boundary and Security Model

### Status

Draft

### Summary

Define the hard boundary between Flurry applications and the host operating system.

### Motivation

Flurry applications must not have implicit OS-level access. A runtime boundary enables permission enforcement and isolation.

### Model

* Applications run inside Flurry Runtime
* All system access is brokered
* Permissions are explicit and revocable

### Invariants

* Apps cannot escape runtime sandbox
* No direct OS API access

### Non-Goals

* Full OS sandbox specification
* App store policy definition

---

## RFC-0007: Determinism Guarantees

### Status

Draft

### Summary

Define what Flurry guarantees—and does not guarantee—about determinism.

### Guarantees

* Deterministic layout
* Deterministic event dispatch
* Deterministic scene structure

### Non-Guarantees

* Bit-perfect pixel output
* Identical GPU driver behavior

### Motivation

Determinism builds developer trust and enables reproducible debugging.

---

## RFC-0008: Early-Stage Scope Discipline

### Status

Draft

### Summary

Define what Flurry explicitly avoids during its foundation phase.

### In-Scope

* Vision locking
* Naming and contracts
* Public API shape
* Deterministic behavior

### Out-of-Scope

* Performance tuning
* Feature parity with browsers
* Advanced animation systems
* Complex text shaping

### Rationale

Early overreach creates architectural debt that cannot be undone.

---

## Closing Notes

These RFCs define Flurry’s *shape*, not its engine. All early implementations are disposable. These documents are not.
