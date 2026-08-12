# WebSocket Protocol

## Purpose

Defines the WebSocket communication model used for real-time communication between `ideal-atc` services and applications where WebSockets are appropriate.

## Scope

This document defines the protocol-level behavior of WebSocket connections.

It does not define:

- Domain message structures
- User-interface behavior
- Internal service implementation
- Network deployment configuration

## Communication Model

The conceptual communication path is:

```text
Producer
   │
   ▼
WebSocket Transport
   │
   ▼
Consumer
```

A producer may publish real-time protocol messages to one or more consumers.

## Message Format

WebSocket messages use the protocol representation defined by the applicable contract.

The exact serialization format and message envelope are to be defined before implementation.

## Connection Lifecycle

A connection consists of:

```text
Connect
   ↓
Establish
   ↓
Active
   ↓
Close
```

The implementation must define behavior for:

- Connection failure
- Unexpected disconnection
- Reconnection
- Invalid messages
- Protocol-version mismatch

## Message Ordering

Ordering requirements must be explicitly defined for message types where ordering affects correctness.

Consumers must not assume ordering unless the applicable protocol guarantees it.

## Delivery

The WebSocket layer does not by itself define durable message delivery.

If a message requires persistence, replay, acknowledgement, or recovery, those requirements must be defined separately.

## Errors

Protocol errors must have a defined representation.

An error should provide sufficient information for the receiving application to determine whether the problem is:

- Invalid input
- Unsupported operation
- Protocol incompatibility
- Temporary failure
- Connection failure

## Keepalive

The connection model must define how liveness is detected.

The exact heartbeat or keepalive mechanism is an implementation decision unless required by a protocol contract.

## Security

Production deployments must define:

- Transport security
- Authentication
- Authorization
- Connection limits
- Input validation

Security requirements are documented separately under:

```text
docs/standards/security.md
```

## Versioning

WebSocket protocol changes must follow the project's protocol versioning rules.

See:

[Versioning](versioning.md)

## Related Documents

- [Protocol Metadata](metadata.md)
- [Protocol Buffers](protobuf.md)
- [Protocol Versioning](versioning.md)
