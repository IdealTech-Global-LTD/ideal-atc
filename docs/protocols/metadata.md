# Protocol Metadata

## Purpose

Defines common metadata concepts used by `ideal-atc` protocol messages.

Metadata provides context about a message independently of the domain payload.

## Scope

This document defines metadata concepts only.

It does not define:

- Domain-specific payloads
- Transport protocols
- Serialization implementations
- Application-specific fields

## Metadata Concepts

### Message Identity

Identifies a message when message-level identification is required.

The exact identifier format is to be defined by the protocol contract.

### Timestamp

Represents the time associated with a message or event.

The protocol contract must define:

- Timestamp format
- Precision
- Time standard
- Whether the timestamp is required

### Source

Identifies the system or component that produced the message.

The source model is to be defined by the protocol contract.

### Protocol Version

Identifies the protocol version used by the message.

Versioning rules are defined in [Versioning](versioning.md).

### Correlation Identifier

Associates related messages or operations when correlation is required.

Its use is defined by the relevant protocol contract.

### Sequence Information

Provides ordering information where message ordering is relevant.

The requirements for sequence numbering are defined by the relevant protocol.

## General Rules

- Metadata must have a clearly defined meaning.
- Metadata must not duplicate domain payload information unnecessarily.
- Required metadata must be explicitly defined by the relevant protocol contract.
- Unknown metadata must not cause a compatible receiver to fail where the underlying protocol supports forward compatibility.
- Metadata semantics must remain consistent across services.

## Protocol Representation

The machine-readable representation of metadata is defined under:

```text
proto/atc/v1/
```

## Related Documents

- [Protobuf](protobuf.md)
- [Versioning](versioning.md)
- [WebSockets](websockets.md)
