# Flurry

**Flurry** is a **Rust-native, GPU-first UI platform** for building deterministic, strongly-typed user interfaces.

Think of it as:

> **A browser-like runtime for compiled UI apps — without HTML, CSS, or JavaScript.**

---

## What Flurry Is

Flurry consists of two parts:

- **Flurry Framework**
  A Rust UI framework used by developers to build Flurry apps.

- **Flurry Runtime**
  A browser-like application that loads, isolates, and renders Flurry apps on a user’s machine.

Together, they form a single, coherent UI system where:

- Types are real
- Layout is deterministic
- Rendering is GPU-first
- Behavior is predictable

---

## What Flurry Is NOT

Flurry is intentionally **not**:

- A web framework
- A replacement for HTML, CSS, or JavaScript
- A browser engine
- A JavaScript runtime
- A WebView wrapper

Flurry does **not** inherit DOM semantics, CSS behavior, or JS reactivity models.

---

## Core Principles

Flurry is built around a few non-negotiable ideas:

- **Deterministic layout**
  Same input → same geometry, always.

- **Strong, real types**
  No string-based attributes or runtime guessing.

- **GPU-first rendering**
  The GPU path defines correctness. CPU rendering exists only as a fallback.

- **Explicit behavior**
  No hidden observers, no magic reactivity, no implicit side effects.

- **Clear separation of concerns**
  Layout, scene construction, and rendering are strictly separated.

---

## Mental Model

Flurry uses a retained-mode UI model:

1. Developer declares UI (via Rust macros/functions)
2. Framework builds a typed UI tree
3. Layout engine computes geometry once
4. Renderer draws the scene (GPU or CPU)

Renderers:

- Do **not** recompute layout
- Do **not** run business logic
- Do **not** mutate state

---

## Example (Conceptual)

```rust
ui! {
    column(padding = 16, gap = 8) {
        text("Login")

        input(id = "email")

        button(on_click = submit) {
            text("Sign In")
        }
    }
}
```

- This expands into a **typed UI tree**
- No runtime string parsing
- No DOM
- No CSS cascade

---

## Why a Runtime Exists

Flurry apps **never run directly on the OS**.

All apps run inside the **Flurry Runtime**, which:

- Sandboxes applications
- Controls access to GPU, input, filesystem, and network
- Enforces explicit permissions
- Prevents app-to-app interference

This provides browser-level safety with native performance.

---

## Project Status

Flurry is in **early foundation stage**.

Current focus:

- Vision locking
- Public UI API design
- RFCs and architectural contracts
- Mock / disposable implementations

APIs are unstable and subject to change.

---

## Contribution Philosophy

- Architectural correctness over speed
- Decisions and contracts matter more than code
- Early implementations are disposable
- Public APIs are treated as long-term commitments

Contributions are welcome, but major changes require alignment with the core vision.

---

## License

Open-source.
License to be finalized.

---

## Bottom Line

Flurry treats UI as a **systems problem**, not a scripting problem.

It is designed for developers who value:

- Predictability
- Strong typing
- Explicit behavior
- Long-term architectural integrity

If that sounds like you, you’re in the right place.
