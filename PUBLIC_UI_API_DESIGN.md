# Flurry — Public UI API Design

## Purpose

Define the **public, stable UI API** that developers will use to build Flurry applications.

This document specifies **what developers write**, not how Flurry renders or executes it internally.

This API is a **contract**:

* Safe to design solo
* Hard to change later
* Independent of rendering backend (WGPU / TinySkia)

---

## Design Goals

* Declarative and readable (HTML-like mental model)
* Strongly typed (Rust-native, no erased types)
* Deterministic layout and behavior
* GPU-friendly by default
* Explicit, predictable, and boring (in a good way)

---

## High-Level Mental Model

Flurry UI is composed of:

1. **Elements** – visible or interactive UI nodes
2. **Attributes** – layout, style, and behavior configuration
3. **Events** – explicit user interactions
4. **UI Tree** – a static description of UI structure

> UI describes *what exists*, not *how it updates*. Internals handle diffing and rendering.

---

## Top-Level Entry Point

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

* `ui! {}` is a macro
* Expands into a **typed UI tree**
* No runtime string parsing

---

## Core Element Set (v0)

### Layout Elements

| Element   | Purpose                   |
| --------- | ------------------------- |
| `column`  | Vertical layout           |
| `row`     | Horizontal layout         |
| `stack`   | Z-axis / overlay layout   |
| `spacer`  | Flexible or fixed spacing |
| `divider` | Visual separator          |

---

### Content Elements

| Element | Purpose               |
| ------- | --------------------- |
| `text`  | Render text           |
| `image` | Render images         |
| `icon`  | Vector or glyph icons |

---

### Input & Control Elements

| Element    | Purpose             |
| ---------- | ------------------- |
| `button`   | Clickable action    |
| `input`    | Text input          |
| `checkbox` | Boolean input       |
| `switch`   | Boolean toggle      |
| `slider`   | Numeric range input |
| `dropdown` | Selection list      |

---

### Structural / Utility Elements

| Element   | Purpose                                      |
| --------- | -------------------------------------------- |
| `scroll`  | Scrollable container                         |
| `surface` | Background, elevation, clipping              |
| `portal`  | Render outside normal tree (dialogs, popups) |

---

## Global Attributes (Available on All Elements)

| Attribute  | Type   | Description                 |
| ---------- | ------ | --------------------------- |
| `id`       | `&str` | Unique identifier           |
| `class`    | `&str` | Style grouping hook         |
| `hidden`   | `bool` | Excluded from layout        |
| `disabled` | `bool` | Disabled interaction        |
| `key`      | `u64`  | Stable identity for diffing |

---

## Layout Attributes

### Box Model

| Attribute | Type                 | Description   |
| --------- | -------------------- | ------------- |
| `padding` | `u32` / `EdgeInsets` | Inner spacing |
| `margin`  | `u32` / `EdgeInsets` | Outer spacing |
| `gap`     | `u32`                | Child spacing |

---

### Sizing

| Attribute   | Type     | Description         |
| ----------- | -------- | ------------------- |
| `width`     | `Length` | Fixed / fill / auto |
| `height`    | `Length` | Fixed / fill / auto |
| `min_width` | `u32`    | Minimum width       |
| `max_width` | `u32`    | Maximum width       |

`Length`:

* `px(u32)`
* `fill()`
* `auto()`

---

### Alignment

| Attribute     | Type    | Description          |
| ------------- | ------- | -------------------- |
| `align_main`  | `Align` | Main-axis alignment  |
| `align_cross` | `Align` | Cross-axis alignment |

`Align`: `start | center | end | stretch`

---

## Style Attributes (v0 – Minimal)

| Attribute    | Type     | Description             |
| ------------ | -------- | ----------------------- |
| `background` | `Color`  | Background color        |
| `color`      | `Color`  | Text / foreground color |
| `radius`     | `u32`    | Corner radius           |
| `border`     | `Border` | Border style            |
| `opacity`    | `f32`    | 0.0 – 1.0               |

---

## Event Attributes

Events are **explicit function bindings**, not strings.

| Event       | Element          | Signature    |
| ----------- | ---------------- | ------------ |
| `on_click`  | button           | `fn()`       |
| `on_input`  | input            | `fn(String)` |
| `on_change` | checkbox, switch | `fn(bool)`   |
| `on_submit` | input            | `fn()`       |

---

## Conditional Rendering

```rust
if is_logged_in {
    text("Welcome")
}
```

* Conditions are evaluated at build time of the UI tree
* Hidden nodes are excluded from layout

---

## Reusable Components

```rust
component LoginButton(on_press: fn()) {
    button(on_click = on_press) {
        text("Sign In")
    }
}
```

* Components are macros or functions
* Fully typed
* No runtime reflection

---

## Dialogs, Popups, Overlays

```rust
portal {
    dialog(open = show_modal) {
        text("Confirm action")
    }
}
```

* `portal` breaks out of normal layout
* Used for dialogs, tooltips, context menus

---

## What This API Intentionally Avoids

* No implicit layout rules
* No CSS selectors or cascading
* No string-based event handlers
* No runtime DOM mutation
* No JS-style reactivity

---

## Stability Policy

* Element names are **frozen early**
* Attribute names are **hard contracts**
* Semantics evolve via RFCs, not breaking changes

---

## Summary

This public UI API is:

* Declarative
* Typed
* Deterministic
* Backend-agnostic

All early engine code is disposable.

This API is not.
