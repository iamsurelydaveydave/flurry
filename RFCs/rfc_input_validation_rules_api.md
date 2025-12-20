# RFC: Input Validation Rules API

## Status
Draft

## Purpose

Define the public API and semantic guarantees for **input validation rules** in Flurry.

This RFC specifies:
- How validation rules are declared
- How they are evaluated
- What guarantees they provide

This document defines **contracts**, not implementation details.

---

## Motivation

Validation is a frequent source of:
- Hidden side effects
- Implicit reactivity
- Non-deterministic behavior
- Tight coupling between inputs and forms

Flurry requires a validation system that is:
- Predictable
- Explicit
- Strongly typed
- Composable

This RFC adopts a **rule-as-function** model inspired by Vuetify, refined to align with Flurry’s retained-mode UI architecture.

---

## Design Principles

1. **Validation rules are pure functions**
2. **Validation produces data, not behavior**
3. **Rules are input-local**
4. **No implicit cross-field access**
5. **Validation is deterministic**

---

## Validation Rule Model

### Rule Definition

A validation rule is a **pure function** that evaluates a single input value.

Conceptual signature:

```
(value: T) -> true | String
```

Where:
- `T` is the input’s value type
- `true` indicates success
- `String` represents a validation error message

Rules must not:
- Mutate state
- Trigger side effects
- Perform IO
- Depend on external mutable data

---

## Rule Execution Semantics

- Rules execute during UI tree evaluation
- Rules run only when:
  - Input value changes, or
  - Input is first evaluated
- Rules execute in declaration order

The first failing rule may short-circuit evaluation.

---

## Validation Result

Each input produces a **validation result** derived from its rules.

### Validation Result States

- **Valid**: all rules returned `true`
- **Invalid**: one or more rules returned a `String`

Only the first error message is surfaced at the input level.

---

## Input-Level API

### Public Properties

- `rules: [Rule<T>]`

Rules are optional. An input with no rules is always valid.

### Guarantees

- Rule evaluation is deterministic
- Invalid rules do not mutate input value
- Validation does not affect layout computation

---

## Interaction with Form Aggregation

- Inputs expose only their validation result
- Inputs do **not** know about forms
- Forms aggregate validation results from descendants

This preserves strict separation of concerns.

---

## Disabled and Read-Only Interaction

### Disabled Inputs

When an input is disabled:
- Rules do not execute
- Input is considered valid

Disabled inputs do not contribute to form invalidation.

---

### Read-Only Inputs

When an input is read-only:
- Rules may execute
- Value does not change
- Validation reflects the current value

This supports review and approval workflows.

---

## Determinism Guarantees

Given the same:
- Input value
- Rule list

Flurry guarantees:
- Identical validation results
- Identical error messages
- Identical form aggregation behavior

No rule execution may depend on runtime ordering or hidden observers.

---

## Error Message Semantics

- Error messages are opaque strings
- Localization and formatting are outside this RFC
- Rendering of error messages is a UI concern

Rules define **what is invalid**, not **how errors appear**.

---

## Non-Goals

This RFC intentionally excludes:

- Async validation
- Cross-field validation
- Form-level validation rules
- Validation throttling or debouncing
- Error display components

These require separate RFCs.

---

## Trade-Offs

### Advantages

- Simple mental model
- Strong typing
- No hidden dependencies
- Easy to test

### Limitations

- No implicit cross-field awareness
- Async validation requires explicit extension

These constraints are intentional.

---

## Compatibility and Stability

- Validation rules are backend-agnostic
- Behavior is identical across GPU and CPU paths
- Rule signatures and semantics are frozen once stabilized

Breaking changes require a new RFC.

---

## Summary

This RFC defines a validation system where:

- Rules are pure functions
- Validation returns data, not side effects
- Inputs own validation
- Forms aggregate results

The system is:
- Deterministic
- Explicit
- Typed
- Scalable

All early implementations are disposable.

This contract is not.

