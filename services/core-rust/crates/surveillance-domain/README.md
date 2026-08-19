# Surveillance Domain

The **Surveillance Domain** is the foundation of the Rust backend for Ideal ATC.

It provides the shared data models that every surveillance service will use, such as aircraft position, altitude, speed, and tracks.

## What this crate does

* Represents aircraft positions
* Represents altitude and speed
* Creates surveillance observations
* Represents tracked aircraft
* Provides common identifiers and timestamps

## What this crate does NOT do

This crate is **not** responsible for:

* Reading ADS-B or radar data
* Network communication (UDP, TCP, WebSockets)
* Conflict detection
* Database storage
* User interface

Those features belong to other Rust crates.

## Folder structure

```text
src/
├── identifier.rs
├── position.rs
├── altitude.rs
├── velocity.rs
├── timestamp.rs
├── observation.rs
├── track.rs
├── quality.rs
├── error.rs
└── lib.rs
```

## How it fits into the system

```text
ADS-B / Radar
      │
      ▼
Observation
      │
      ▼
Track Engine
      │
      ▼
Aircraft Track
```

This crate defines the **Observation** and **Track** models used throughout the surveillance engine.

## Current status

* [x] Domain models
* [x] Unit tests
* [x] Documentation
* [ ] ADS-B ingestion
* [ ] Track engine
* [ ] Conflict detection
