# RFC: Form Interaction & Validation State Model

## Status
Draft

## Purpose

Define the interaction, validation, and state aggregation model for `form` elements in Flurry.

This RFC specifies **what semantic guarantees a form provides to application developers**, not how validation or propagation is implemented internally.

The form API defined here is a **public contract** and must remain stable once adopted.

---

## Motivation

Forms are a central coordination primitive in UI systems.

Without a well-defined model, forms tend to:
- Become implicit data owners
- Hide validation logic in side effects
- Rely on observer chains or watchers
- Introduce non-deterministic behavior

Flurry requires a form model that is:
- Deterministic
- Explicit
- Strongly typed
- Easy to reason about

This RFC adopts a model inspired by Vuetify’s `v-form`, refined to fit Flurry’s retained-mode, typed architecture.

---

## Design Principles

1. **Forms do not own input values**
2. **Forms aggregate validation state**
3. **Interaction control is declarative and hierarchical**
4. **All state is explicit and typed**
5. **No hidden mutation or observers**

---

## Form Role

A `form` is a **state boundary and aggregator**, not a data container.

Its responsibilities are limited to:
- Aggregating validation state from descendant inputs
- Controlling interaction state for descendant inputs
- Exposing a single, stable validity signal

The form does **not**:
- Store input values
- Mutate child state
- Perform side effects

---

## Public Form Properties

### 1. `value: bool`

Represents the **aggregated validity state** of the form.

#### Semantics

- `true` if and only if **all descendant inputs are valid**
- `false` if **any input reports a validation error**
- Derived state, not mutable storage

This mirrors Vuetify’s `v-form` model, where the form’s value is a validity flag.

#### Guarantees

- Deterministic
- Stable for a given UI tree and input state
- Updated only during UI tree evaluation

---

### 2. `disabled: bool`

Controls **hard interaction locking** for the form and all descendants.

#### Semantics

When `disabled = true`:
- Inputs cannot receive focus
- Input values cannot be modified
- Validation does not run
- Events do not fire

This state propagates unconditionally to all descendant elements.

#### Use Cases

- Submitting state
- Permission-based access control
- External process locking

---

### 3. `read_only: bool`

Controls **value mutability without disabling interaction**.

#### Semantics

When `read_only = true`:
- Inputs may receive focus
- Text may be selected
- Input values cannot be changed
- Validation may still run

This mirrors native platform behavior and must not be conflated with `disabled`.

#### Use Cases

- Review screens
- Approval workflows
- View-only forms

---

## Interaction State Matrix

| State        | Focus | Value Change | Validation | Events |
|-------------|-------|--------------|------------|--------|
| Normal      | Yes   | Yes          | Yes        | Yes    |
| Read-only   | Yes   | No           | Yes        | Yes    |
| Disabled    | No    | No           | No         | No     |

---

## Propagation Rules

1. `disabled = true` overrides all descendant interaction state
2. `read_only = true` propagates unless explicitly overridden (rare)
3. Descendants may **not** re-enable themselves if a parent form is disabled
4. Propagation occurs during UI tree evaluation, not at render time

These rules are strict and non-negotiable.

---

## Validation Model

### Input-Level Validation

- Inputs define validation rules as pure functions
- Rules return:
  - `true` for success
  - `String` for validation error

### Form-Level Aggregation

- The form observes validation results from descendant inputs
- `form.value` is computed as:

```
true  if all inputs are valid
false if any input returns an error
```

The form does not inspect input values, only validation outcomes.

---

## Determinism Guarantees

Given the same:
- UI tree structure
- Input values
- Validation rules

The following are guaranteed:
- Identical `form.value`
- Identical interaction behavior
- Identical propagation of disabled and read-only states

No runtime observers, watchers, or hidden re-evaluation paths are permitted.

---

## Non-Goals

This RFC intentionally does **not** define:

- How validation rules are executed internally
- Async validation behavior
- Cross-form validation
- Submission handling
- Error message rendering

These concerns belong to separate RFCs.

---

## Trade-Offs

### Advantages

- Simple mental model
- Strong typing
- Clear separation of concerns
- Deterministic behavior

### Limitations

- Less dynamic than observer-based systems
- Requires explicit state wiring by developers

These trade-offs are intentional.

---

## Compatibility and Stability

- This API is backend-agnostic
- Compatible with GPU and CPU renderers
- Once stabilized, property names and semantics are frozen

Breaking changes require a new RFC and explicit migration path.

---

## Summary

This RFC defines a form model where:

- `value` represents aggregated validity
- `disabled` enforces hard interaction lock
- `read_only` allows safe, view-only interaction

The model is:
- Deterministic
- Typed
- Explicit
- Scalable

This establishes `form` as a **first-class, predictable state boundary** in Flurry.

All early implementations are disposable.

This contract is not.

