# RFC: Form Submission & Lifecycle Events

## Status
Draft

## Purpose

Define the submission semantics and lifecycle events for `form` elements in Flurry.

This RFC specifies:
- How forms are submitted
- What lifecycle events exist
- When events are allowed to fire

This document defines **behavioral contracts**, not rendering or transport mechanisms.

---

## Motivation

Form submission is often implemented using:
- Implicit browser behavior
- Hidden event bubbling
- Side-effect-driven callbacks

These approaches conflict with Flurry’s goals of:
- Determinism
- Explicit state flow
- Predictable execution

Flurry requires a form submission model that is:
- Explicit
- Typed
- Deterministic
- Compatible with retained-mode UI

---

## Design Principles

1. **Submission is an explicit event**
2. **No implicit browser-style behavior**
3. **Lifecycle stages are finite and ordered**
4. **Events do not mutate UI directly**
5. **Forms do not own business logic**

---

## Form Submission Model

### Submission Trigger

A form submission may be triggered only by:
- An explicit submit event
- A submit-capable control (e.g., submit button)

There is no implicit submission via key presses or focus changes.

---

## Public Form Events

### 1. `on_submit`

Signature (conceptual):

```
on_submit: fn()
```

#### Semantics

- Fired when a submit action is invoked
- Fires **only if**:
  - `form.disabled == false`
  - `form.value == true` (form is valid)

If these conditions are not met, the event does not fire.

---

### 2. `on_invalid_submit`

Signature (conceptual):

```
on_invalid_submit: fn()
```

#### Semantics

- Fired when a submit action is invoked
- Fires **only if**:
  - `form.disabled == false`
  - `form.value == false`

Allows explicit handling of invalid submission attempts.

---

### 3. `on_reset`

Signature (conceptual):

```
on_reset: fn()
```

#### Semantics

- Fired when a reset action is explicitly triggered
- Does not implicitly clear input values

Reset behavior is application-defined.

---

## Submission Lifecycle

Submission follows a strict lifecycle:

1. **Submit Requested**
2. **Validation Check**
3. **Event Dispatch**

No other lifecycle stages are permitted.

---

## Lifecycle Invariants

- Submission does not recompute layout
- Submission does not mutate input values
- Submission does not trigger validation re-runs
- Events fire at most once per submit action

---

## Interaction with Disabled and Read-Only States

### Disabled Form

When `disabled = true`:
- Submit events do not fire
- Reset events do not fire

---

### Read-Only Form

When `read_only = true`:
- Submit events may fire
- Input values remain unchanged

This supports review and approval workflows.

---

## Determinism Guarantees

Given the same:
- UI tree
- Input values
- Validation state

Flurry guarantees:
- Identical submit eligibility
- Identical event dispatch behavior

No implicit retries, bubbling, or cancellation.

---

## Error Handling

- Submission events do not carry validation errors
- Error presentation is a UI concern
- Business logic errors are handled outside the form

---

## Non-Goals

This RFC intentionally excludes:

- Async submission handling
- Network request lifecycle
- Loading or pending states
- Keyboard shortcuts (e.g., Enter-to-submit)
- Automatic value clearing

These require separate RFCs.

---

## Trade-Offs

### Advantages

- Simple and explicit model
- No hidden side effects
- Strong separation of concerns

### Limitations

- Less convenience than browser defaults
- Requires explicit wiring by developers

These trade-offs are intentional.

---

## Compatibility and Stability

- Backend-agnostic
- Identical behavior across GPU and CPU paths
- Event names and semantics are frozen once stabilized

Breaking changes require a new RFC.

---

## Summary

This RFC defines a form submission system where:

- Submission is explicit
- Validation gates submission
- Lifecycle stages are minimal and ordered
- Events are deterministic and typed

This establishes a predictable, non-magical form submission model for Flurry.

All early implementations are disposable.

This contract is not.