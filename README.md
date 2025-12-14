# Flurry

## Overview

Flurry is a **GPU-first, strongly typed UI ecosystem** built with **Rust**.

Flurry follows the same fundamental model as the web:

* A **browser** renders websites
* **Flurry Runtime** renders Flurry applications

Flurry consists of **two distinct but tightly integrated parts**:

1. **Flurry Framework** – a Rust UI framework used by developers to build Flurry applications
2. **Flurry Runtime** – a browser-like application installed on a user’s machine that loads and runs Flurry applications

Together, they form a platform where UI applications are **compiled**, **typed**, and **rendered deterministically**, without relying on JavaScript, browser engines, or erased types.

Flurry exists to solve a fundamental gap in today’s frontend ecosystem: the lack of a platform where **types, runtime, execution model, and rendering** are part of a single, coherent system.

Flurry is not a JavaScript framework. It does not rely on erased types, browser semantics, or managed runtimes. Instead, it treats UI as a **compiled application problem**, not a scripting problem.

---

## Core Goals

* **Real Types** – No erased or tooling-only types
* **Deterministic Execution** – Predictable behavior across environments
* **GPU-First Rendering** – Modern rendering as the baseline, not an optimization
* **Native Performance** – Compiled binaries, minimal runtime overhead
* **Single Mental Model** – One language, one system, one execution model

---

## Design Principles

### 1. GPU-First Architecture

Flurry assumes GPU availability by default. Modern laptops, desktops, and mobile devices all ship with integrated GPUs capable of handling UI workloads.

* Primary renderer: **WGPU**
* Backends: Vulkan, Metal, DX12, WebGPU
* Performance targets and visual correctness are defined on the GPU path

### 2. Explicit CPU Fallback (Not a Peer)

A CPU renderer exists for:

* Headless environments
* CI and testing
* Servers without GPU access
* Safe fallback on GPU initialization failure

CPU rendering is **best-effort and may degrade gracefully**. It is not a design driver.

* CPU renderer: **TinySkia** (software rasterization)

---

## Rendering Model

Flurry enforces a strict separation of concerns:

```
Layout Engine
   ↓
Scene / Display List
   ↓
Renderer Backend
   ├─ WGPU (GPU)
   └─ TinySkia (CPU)
```

* Layout and geometry are computed once
* Rendering backends are interchangeable
* Renderers never recompute layout or business logic

This guarantees **layout determinism** and **visual consistency** across renderers.

---

## Consistency Guarantees

Flurry guarantees:

* Identical layout and geometry between GPU and CPU
* Identical interaction behavior
* Visually equivalent output

Flurry does **not** guarantee:

* Bit-perfect pixel output
* Identical antialiasing or subpixel rendering

This matches industry-standard practices (Flutter, browsers, game engines).

---

## Why Flurry Exists

Current UI stacks suffer from:

* Erased or tooling-only types
* Runtime vs compile-time mismatch
* Fragmented languages and build pipelines
* Unpredictable performance characteristics

Flurry is designed to:

* Treat UI as a systems problem
* Apply backend engineering principles to UI
* Enable long-term correctness and maintainability

---

---

## Target Use Cases

* Desktop applications
* Internal tools and admin dashboards
* High-performance UI applications
* Embedded and edge UIs
* Headless rendering and testing
* **Hosted Flurry applications rendered inside the Flurry Runtime**

Flurry is **not** intended for:

* SEO-first websites
* Document-centric web pages
* Lightweight static sites

---

### Why a Runtime Is Required

Flurry is designed as a **runtime-based platform** (similar to a browser) to ensure that Flurry applications **cannot directly exploit or compromise the host machine**.

Without a runtime boundary, compiled UI applications would require broad OS-level access, making it difficult to:

* Enforce permissions
* Isolate applications from each other
* Prevent malicious or unintended system access

The **Flurry Runtime exists as a security boundary**.

It is responsible for:

* Sandboxing Flurry applications
* Controlling access to system resources (GPU, input, filesystem, network)
* Enforcing explicit permission policies
* Preventing applications from escaping their execution context

This model mirrors the security guarantees of modern browsers, while retaining native performance and strong typing.

---

---

---

## Framework, Runtime, and Application Model

### Flurry Framework (Developer Side)

* A Rust-based UI framework
* Used to build **Flurry applications**
* Produces compiled Flurry app artifacts
* Enforces strong typing, deterministic layout, and GPU-first rendering

Flurry apps are authored and compiled by developers using Rust and the Flurry Framework.

---

### Flurry Runtime (User Side)

The **Flurry Runtime** is a browser-like application that users install on their machine.

It is responsible for:

* Discovering, loading, and executing Flurry applications
* Providing GPU access, input handling, and window management
* Enforcing permissions and isolation between apps
* Managing application lifecycle and updates

Unlike traditional desktop applications:

* Users install **Flurry once**
* Individual Flurry apps do **not** require OS-level installation

---

### Application Distribution Model

* Flurry applications can be hosted in **git-based repositories** (GitHub, GitLab, or self-hosted Git servers)
* Publishing an app is as simple as pushing to a repository
* The Flurry Runtime fetches, verifies, and runs applications on demand
* Applications can be updated independently of the runtime

This model removes traditional packaging and installer complexity while preserving developer control.

It mimics the **deployment simplicity of the web**, without requiring HTML, CSS, or JavaScript.

---

### Decoupled Backend Architecture

* Flurry apps are **frontend-only** artifacts
* Backends are hosted independently (VPS, cloud, on-prem)
* Apps communicate with their backends over standard protocols (HTTP, gRPC, WebSocket)

This model enables:

* Zero-install user experience
* Clear separation of UI and backend concerns
* Stronger security and permission control

---

## Governance Model

Flurry follows a **stewarded open-source model**.

* The core team owns the vision and architectural direction
* Contributions are welcome within defined boundaries
* Public APIs and major architectural changes require maintainer approval

This model preserves long-term coherence and prevents fragmentation.

---

## Team Structure

* **Founder / Steward** – Vision and product direction
* **Rust Systems Lead** – Core architecture and safety
* **WGPU Rendering Lead** – GPU pipeline and rendering model
* **Supporting Developers** – Feature implementation under guidance
* **Contributors / Volunteers** – Scoped contributions under review

---

## Contribution Philosophy

Flurry values:

* Technical correctness over speed
* Explicit ownership
* Clear boundaries
* Small, reviewable changes

Contributions that diverge from the project direction may be declined, even if technically correct.

---

---

## Project Status

Flurry is in **early foundation stage**.

Current focus:

* Core architecture
* Rendering abstraction
* GPU-first pipeline
* Deterministic layout system

APIs are unstable and subject to change.

---

## License

Flurry is open-source under a permissive license (TBD).

---

## Vision

Flurry aims to become a **first-class UI platform** where developers can build interfaces with the same confidence, rigor, and correctness they expect from backend systems.

This project prioritizes **long-term architectural integrity over short-term convenience**.
