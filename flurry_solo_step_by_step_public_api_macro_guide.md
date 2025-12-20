# Flurry — Solo Step-by-Step Public API & Macro Guide

This guide tells you **exactly what to work on, in order**, as a solo founder.  
It deliberately avoids GPU, WGPU, and performance work.  
The goal is to lock **contracts, naming, and mental model**.

---

## Phase 0 — Mental Reset (Do This First)

### Objective
Align your work with what is **safe to do solo**.

### Rules
- You are designing **what developers write**, not how Flurry renders.
- Code is disposable; **API shape is not**.
- If something feels engine-level, stop.

Deliverable:
- You are clear that your scope is **Public UI API + macro expansion shape**.

---

## Phase 1 — Lock the Public UI Vocabulary

### Objective
Freeze the **words** of Flurry.

### 1.1 Define the Core Element Set (v0)

Create a single Rust file or document that lists **only names**.

**Layout elements**
- column
- row
- stack
- spacer
- divider

**Content elements**
- text
- image
- icon

**Input / control elements**
- button
- input
- checkbox
- switch
- slider
- dropdown

**Structural / utility elements**
- scroll
- surface
- portal
- form

Rules:
- No behavior yet
- No implementation
- No feature discussion

Deliverable:
- A frozen list of element identifiers

---

### 1.2 Define Global Attributes (Apply to ALL Elements)

Write these as if they were a Rust struct, but **do not implement**.

Global attributes:
- id: &str
- class: &str
- key: u64
- hidden: bool
- disabled: bool

Rules:
- Attribute names must be boring and obvious
- No CSS-like behavior

Deliverable:
- A documented global attribute list

---

## Phase 2 — Layout & Sizing Contract (No Math Yet)

### Objective
Define **what options exist**, not how they work internally.

### 2.1 Box Model Attributes

- padding: u32 | EdgeInsets
- margin: u32 | EdgeInsets
- gap: u32

### 2.2 Size Model

Define a `Length` enum conceptually:

- px(u32)
- fill()
- auto()

Used by:
- width
- height
- min_width
- max_width

Rules:
- No percentages
- No content-based magic

Deliverable:
- A stable size vocabulary

---

### 2.3 Alignment Vocabulary

Define alignment enums:

- align_main: start | center | end | stretch
- align_cross: start | center | end | stretch

Deliverable:
- Explicit axis-based alignment model

---

## Phase 3 — Event & Interaction Contract

### Objective
Lock **what events exist and when they fire**.

### 3.1 Event Naming

- on_click → button
- on_input → input
- on_change → checkbox, switch
- on_submit → form
- on_invalid_submit → form
- on_reset → form

Rules:
- Events are functions, not strings
- Events never mutate layout

Deliverable:
- Event-to-element mapping table

---

## Phase 4 — Form & Validation Model (Already Designed, Now Integrate)

### Objective
Integrate existing RFCs into the public API shape.

### 4.1 Form Properties

- value: bool (aggregated validity)
- disabled: bool
- read_only: bool

### 4.2 Input Validation Rules

Conceptual rule signature:

(value: T) -> true | String

Rules:
- Pure functions only
- Input-local only

Deliverable:
- Public-facing form and input API description

---

## Phase 5 — Conditional & Structural Constructs

### Objective
Define **what is allowed structurally**.

### 5.1 Conditional Rendering

Example (conceptual):

if is_logged_in {
  text("Welcome")
}

Rules:
- Condition evaluated during UI build
- Hidden nodes do not exist in layout

---

### 5.2 Portal / Overlay Model

portal {
  dialog(open = show) {
    text("Confirm")
  }
}

Rules:
- portal escapes normal layout tree
- Still part of UI tree

Deliverable:
- Structural escape hatch definition

---

## Phase 6 — The `ui!` Macro (This Is Where Macros Come In)

### Objective
Define **macro expansion shape**, not macro internals.

### 6.1 What the `ui!` Macro Does

Conceptually:

ui! { ... }

Expands into:
- A typed UI tree
- Immutable node structures
- No runtime parsing

### 6.2 What the Macro Must Support

- Nested element blocks
- Attributes as named arguments
- Children as block bodies
- Inline conditionals

### 6.3 What the Macro Must NOT Do

- No layout computation
- No rendering
- No mutation

Deliverable:
- A written description of macro expansion stages

---

## Phase 7 — Mock Implementation (Optional, Safe)

### Objective
Validate your mental model without committing to engine code.

Allowed:
- Structs like `UiNode`
- A tree builder
- Printing node hierarchy

Forbidden:
- WGPU
- Unsafe Rust
- Optimization

Deliverable:
- A throwaway prototype

---

## Phase 8 — Stop and Freeze

### Objective
Prevent accidental scope creep.

Checklist:
- [ ] Element names frozen
- [ ] Attribute names frozen
- [ ] Event names frozen
- [ ] Macro responsibilities documented

When this is done:
- You are **ready for engine work later**
- You can safely onboard contributors

---

## Final Reminder

If you ever ask:
> “Should I code this now?”

Ask instead:
> “Is this a contract or an implementation?”

Only contracts belong in this phase.

---

End of guide.