# Protocol Versioning

## Purpose

Defines how communication contracts evolve within `ideal-atc`.

## Scope

This document applies to protocol contracts used between system components.

## Version Structure

Protocol definitions are versioned explicitly.

The current protocol namespace is:

```text
atc.v1
```

The version represents the compatibility boundary of the protocol contract.

## Compatibility

Changes should preserve compatibility where possible.

Compatible changes may include:

- Adding new fields
- Adding optional information
- Adding new message types

Potentially incompatible changes include:

- Removing existing fields
- Changing field meaning
- Reusing field numbers
- Changing the type or semantics of an existing field
- Changing required behavior

## Field Removal

A removed field must not have its field number reused for an unrelated field.

Where supported by the protocol definition, removed fields should be reserved.

## Message Evolution

Existing messages should evolve without changing the meaning of existing fields.

If the required change cannot remain compatible, a new message or protocol version should be considered.

## Version Changes

A new protocol version may be introduced when compatibility cannot reasonably be maintained.

Example:

```text
atc.v1
   │
   │ incompatible contract change
   ▼
atc.v2
```

The migration strategy must be documented before introducing the new version.

## Deprecation

Deprecated messages or fields must be documented.

Deprecation should identify:

- What is deprecated
- Replacement functionality
- Compatibility period
- Removal plan

## Generated Code

Generated language bindings must correspond to a specific protocol version.

Implementations must not mix incompatible generated contracts.

## Source of Truth

The versioned `.proto` definitions are authoritative.

Documentation and implementations must conform to them.

## Related Documents

- [Protocol Buffers](protobuf.md)
- [Protocol Metadata](metadata.md)
- [WebSockets](websockets.md)
