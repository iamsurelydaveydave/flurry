# flurry_ui

A typed AST for Flurry's UI language - not a framework, just a contract.

## Overview

`flurry_ui` provides a typed, deterministic interface for describing user interfaces as abstract syntax trees. It focuses purely on structure and type safety, with no rendering, layout computation, or runtime behavior.

## Features

- **Typed AST**: All UI elements are represented as strongly-typed structures
- **Compile-time Safety**: Invalid attribute combinations fail at compile time
- **Zero Runtime**: No dynamic dispatch, no string-based keys, no runtime parsing
- **Minimal**: Only core elements (column, row, text, button, input)

## Structure

```
src/
├── lib.rs              # Public API surface
├── node.rs             # UiNode tree structure
├── elements/           # UI element definitions
│   ├── column.rs
│   ├── row.rs
│   ├── text.rs
│   ├── button.rs
│   └── input.rs
├── attributes/         # Typed attributes
│   ├── layout.rs
│   ├── size.rs
│   └── common.rs
├── events.rs           # Event type definitions
└── macros.rs           # ui! macro implementation
```

## Usage

```rust
use flurry_ui::ui;

fn handle_click() {
    // Click handler logic
}

let ui_tree = ui! {
    column(padding = 16, gap = 8) {
        text("Login Form")
        button(on_click = handle_click) {
            text("Sign In")
        }
    }
};
```

## Design Principles

1. **Boring is Good**: No clever abstractions, just data structures
2. **Delete-Safe**: Can be completely reimplemented from RFCs
3. **Type-First**: Invalid combinations should not compile
4. **No Magic**: No defaults, no runtime dispatch, no string matching

## What This Crate Does NOT Do

- Layout computation
- Rendering
- State management
- Event handling (just type definitions)
- Runtime validation
- Dynamic content

These concerns belong in other crates (`flurry_layout`, renderers, etc.).
