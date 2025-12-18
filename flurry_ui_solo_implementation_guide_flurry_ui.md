# Flurry UI — Solo Implementation Guide (flurry_ui)

> Scope: **flurry_ui only**  
> Goal: Define Flurry’s **public UI language** as a typed, deterministic, boring contract.  
> Non-goal: Layout, rendering, GPU, state mutation, reactivity.

---

## 0. Ground Rules (Read Once)

- `flurry_ui` is a **typed AST**, not a framework
- Everything here is **public API**
- All code must be **safe to delete and re-implement** from RFCs
- No behavior, no logic, no rendering assumptions

If you feel clever → stop.

---

## 1. Create the Crate Skeleton (Contract Commit)

Create this exact structure and commit it **before writing code**.

```
crates/flurry_ui/
├─ src/
│  ├─ lib.rs
│  ├─ node.rs
│  ├─ elements/
│  │  ├─ mod.rs
│  │  ├─ column.rs
│  │  ├─ row.rs
│  │  ├─ text.rs
│  │  ├─ button.rs
│  │  └─ input.rs
│  ├─ attributes/
│  │  ├─ mod.rs
│  │  ├─ layout.rs
│  │  ├─ size.rs
│  │  └─ common.rs
│  ├─ events.rs
│  └─ macros.rs
```

**Rule**: file existence is part of the API contract.

---

## 2. lib.rs — Public Surface Only

Purpose: define what is officially supported.

```rust
pub mod node;
pub mod elements;
pub mod attributes;
pub mod events;
pub mod macros;

pub use macros::ui;
```

Rules:
- If it is not re-exported here, it is not public API
- No private helper modules

---

## 3. node.rs — UI Tree Definition

Purpose: represent **structure**, nothing else.

```rust
use crate::elements::Element;

pub struct UiNode {
    pub element: Element,
    pub children: Vec<UiNode>,
}
```

Rules:
- Immutable after construction
- No IDs auto-generated
- No traversal helpers

This mirrors the RFC UI Tree exactly.

---

## 4. elements/ — One File per Element

### 4.1 elements/mod.rs

```rust
pub mod column;
pub mod row;
pub mod text;
pub mod button;
pub mod input;

pub enum Element {
    Column(column::Column),
    Row(row::Row),
    Text(text::Text),
    Button(button::Button),
    Input(input::Input),
}
```

Rules:
- No trait objects
- No shared base traits
- Enum is closed and explicit

---

### 4.2 Example Element — column.rs

```rust
use crate::attributes::layout::Layout;

pub struct Column {
    pub layout: Layout,
}
```

Other element files follow the same pattern:
- **Struct only**
- No methods
- No defaults

---

## 5. attributes/ — Typed Attributes Only

### 5.1 attributes/mod.rs

```rust
pub mod layout;
pub mod size;
pub mod common;
```

---

### 5.2 layout.rs

```rust
pub struct Layout {
    pub padding: u32,
    pub gap: u32,
}
```

---

### 5.3 common.rs

```rust
pub struct Common {
    pub id: Option<&'static str>,
    pub disabled: bool,
    pub hidden: bool,
}
```

Rules:
- No HashMaps
- No string-based keys
- Every attribute is compile-time known

---

## 6. events.rs — Explicit Event Contracts

Purpose: define **what events exist**, not how they fire.

```rust
pub struct OnClick(pub fn());
pub struct OnInput(pub fn(String));
```

Rules:
- Typed function signatures
- No bubbling
- No string dispatch

---

## 7. macros.rs — ui! Macro (Most Important Part)

Purpose: convert declarative syntax into a typed `UiNode` tree **at compile time**.

### 7.1 Target Syntax

```rust
ui! {
    column(padding = 16, gap = 8) {
        text("Login")
        button(on_click = submit) {
            text("Sign In")
        }
    }
}
```

### 7.2 Output Shape (Conceptual)

```rust
UiNode {
    element: Element::Column(...),
    children: vec![ ... ],
}
```

### 7.3 Macro Rules

- No runtime parsing
- No string matching
- Invalid attributes **must not compile**
- Minimal supported syntax first

### 7.4 Start Minimal

Support initially:
- `column`
- `text`
- `button`
- children blocks

Ignore for now:
- conditionals
- components
- portals
- loops

---

## 8. Compile-Fail Safety Tests (Mandatory)

Your API is only correct if **bad code fails to compile**.

These must fail:

```rust
column(color = "red") {}
button(on_click = "submit")
text(123)
```

If any of these compile, your macro is unsafe.

---

## 9. What Must NOT Exist in flurry_ui

Never add:
- Layout computation
- Rendering logic
- State mutation
- Defaults or magic values
- Runtime dispatch

If you feel tempted, you are in the wrong crate.

---

## 10. Stop Condition

You stop working on `flurry_ui` when:
- API matches the RFCs
- ui! macro builds trees correctly
- The code feels boring and strict

Boredom is success.

---

## 11. Mental Check

Ask yourself:

> “Can I delete this entire crate and re-implement it purely from the RFCs?”

If yes → correct
If no → you went too far

---

## 12. Next Steps (Do Not Jump Ahead)

After `flurry_ui`:
1. flurry_layout
2. mock renderer
3. expert review

Nothing else belongs before those.
