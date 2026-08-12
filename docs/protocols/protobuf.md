# Protocol Buffers

## Purpose

Defines the rules for using Protocol Buffers as an inter-service contract within `ideal-atc`.

## Scope

This document defines project-level conventions for `.proto` files.

It does not define individual domain messages.

## Location

Protocol definitions are stored under:

```text
proto/
└── atc/
    └── v1/
```

## Package Naming

Protocol packages use the project namespace followed by the protocol version.

Example:

```text
atc.v1
```

Domain-specific message definitions are separated into appropriate `.proto` files.

## Message Design

Messages should:

- Represent a clearly defined domain concept.
- Have a single clear responsibility.
- Use explicit field names.
- Avoid unnecessary duplication.
- Prefer stable contracts over implementation-specific structures.

## Field Numbers

Once a field number has been published, it must not be reused for a different field.

Removed fields should be reserved where appropriate.

## Field Compatibility

Changes to an existing message must preserve compatibility according to the project's versioning policy.

Potentially breaking changes require a defined protocol-version change.

## Enumerations

Enums should:

- Have explicitly defined values.
- Preserve existing numeric values.
- Reserve removed values where appropriate.
- Avoid reusing previously assigned values.

## Optional Data

Fields that are not guaranteed to be present must have clearly defined presence semantics.

The protocol definition must distinguish between:

- Required domain information
- Optional information
- Unknown information

## Units

Physical quantities must have explicitly defined units.

Units must not be inferred from field names alone.

The authoritative domain conventions are documented in:

```text
docs/domains/units.md
```

## Timestamps

Timestamp semantics must be explicitly defined.

A timestamp must identify what event or state it represents.

## Generated Code

Generated language bindings are derived from the `.proto` definitions.

Generated code must not be manually modified.

## Source of Truth

The `.proto` files are the authoritative machine-readable protocol contracts.

Documentation may explain the contracts but must not contradict them.

## Related Documents

- [Protocol Metadata](metadata.md)
- [Protocol Versioning](versioning.md)
- [WebSockets](websockets.md)
