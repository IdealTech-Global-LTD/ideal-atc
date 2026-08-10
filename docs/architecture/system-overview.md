# Ideal ATC — System Overview

## 1. Purpose

Ideal ATC is a modular Air Traffic Management (ATM) software platform designed to ingest, process, correlate, display, record, and replay aviation surveillance and operational data.

The system is designed around clearly separated subsystems so that surveillance sensors, processing services, controller applications, and operational services can evolve independently.

The architecture supports:

- Primary and secondary surveillance integration
- ADS-B surveillance
- Mode S / SSR surveillance
- MLAT surveillance
- ASTERIX-based surveillance feeds
- Flight data processing
- Track processing and fusion
- Conflict detection and alerting
- Airspace management
- Weather information
- Controller Human-Machine Interfaces (HMI)
- Supervisor applications
- Flight-data applications
- Recording and replay
- Simulation and training

The architecture is designed so that simulated data can be replaced by real surveillance sources without requiring changes to the core ATM domain model.

---

# 2. System Context

At the highest level, Ideal ATC sits between external aviation data sources and operational applications.

```text
                         EXTERNAL SYSTEMS
                              │
          ┌───────────────────┼────────────────────┐
          │                   │                    │
          ▼                   ▼                    ▼
      SURVEILLANCE        FLIGHT DATA           WEATHER
       SOURCES             SOURCES              SOURCES
          │                   │                    │
          └───────────────────┼────────────────────┘
                              │
                              ▼
                    ┌───────────────────┐
                    │   IDEAL ATC CORE   │
                    │                   │
                    │ Surveillance      │
                    │ Flight Data       │
                    │ ATM Services      │
                    │ Airspace          │
                    │ Conflict Detection│
                    │ Recording         │
                    └─────────┬─────────┘
                              │
                 ┌────────────┼────────────┐
                 │            │            │
                 ▼            ▼            ▼
            CONTROLLER    FLIGHT DATA   SUPERVISOR
               HMI           HMI           HMI
                 │
          ┌──────┴──────┐
          ▼             ▼
         WEB          DESKTOP
```

---

# 3. Surveillance System

Surveillance is a first-class subsystem of Ideal ATC.

It is responsible for receiving surveillance observations, decoding external surveillance protocols, validating observations, maintaining tracks, correlating observations from multiple sources, and publishing authoritative track state.

```text
                    SURVEILLANCE SOURCES
                            │
       ┌────────────┬───────┼────────┬────────────┐
       │            │       │        │            │
       ▼            ▼       ▼        ▼            ▼
      PSR          SSR    Mode S    ADS-B        MLAT
       │            │       │        │            │
       └────────────┴───────┼────────┴────────────┘
                            │
                            ▼
                   SURVEILLANCE INPUT
                            │
                            ▼
                  PROTOCOL ADAPTERS
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
          ASTERIX                     Other Input
       Cat 048/062/021                  Adapters
              │                           │
              └─────────────┬─────────────┘
                            ▼
                 OBSERVATION NORMALIZATION
                            │
                            ▼
                    TRACK PROCESSING
                            │
             ┌──────────────┼──────────────┐
             │              │              │
             ▼              ▼              ▼
       Track Update     Track Fusion    Track Quality
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                    AUTHORITATIVE TRACKS
```

### 3.1 Primary Surveillance Radar

Primary Surveillance Radar (PSR) provides surveillance based on reflected radio energy and does not require the aircraft to provide a transponder response.

The architecture treats PSR as a surveillance source rather than embedding PSR-specific logic into the track engine.

```text
PSR
 │
 ▼
PSR Interface / Adapter
 │
 ▼
Surveillance Observation
 │
 ▼
Track Processing
```

The physical radar system and its proprietary hardware interfaces remain outside the core Ideal ATC domain.

---

### 3.2 Secondary Surveillance Radar

Secondary Surveillance Radar (SSR) obtains aircraft responses through transponder interrogation.

The resulting surveillance data enters Ideal ATC through the appropriate surveillance interface and is normalized before reaching the tracking subsystem.

```text
SSR / Mode S
     │
     ▼
Surveillance Adapter
     │
     ▼
Normalized Observation
     │
     ▼
Track Processing
```

---

### 3.3 ADS-B

ADS-B provides aircraft-broadcast surveillance information.

ADS-B may enter the system through:

- Ground receiver hardware
- Network feeds
- ASTERIX Cat 021
- Simulator-generated traffic
- Other supported surveillance gateways

The ADS-B source must not bypass the canonical surveillance model.

```text
ADS-B Receiver
      │
      ▼
ADS-B / ASTERIX Adapter
      │
      ▼
Normalized Observation
      │
      ▼
Track Processing
```

---

### 3.4 MLAT

Multilateration provides position information derived from multiple receiving stations.

MLAT observations follow the same normalization path as other surveillance sources.

```text
MLAT System
     │
     ▼
MLAT Adapter
     │
     ▼
Normalized Observation
     │
     ▼
Track Processing
```

---

# 4. Surveillance Protocol Layer

External surveillance protocols must be isolated from the internal domain model.

For example:

```text
ASTERIX Cat 021
ASTERIX Cat 048
ASTERIX Cat 062
ADS-B feed
Simulator feed
       │
       ▼
Protocol Adapter
       │
       ▼
Canonical Surveillance Observation
       │
       ▼
Track Engine
```

The track engine must not depend directly on the representation of an individual external protocol.

This allows additional surveillance sources to be introduced without redesigning the tracking architecture.

---

# 5. Track Processing

The surveillance subsystem transforms individual observations into coherent aircraft tracks.

```text
Observation
     │
     ▼
Validation
     │
     ▼
Association
     │
     ▼
Track Update
     │
     ▼
Track Prediction
     │
     ▼
Track Quality
     │
     ▼
Track Fusion
     │
     ▼
Authoritative Track
```

A track represents the system's current operational representation of an aircraft or surveillance target.

The track model is defined independently from any particular radar, ADS-B receiver, or external protocol.

---

# 6. ATM Processing

Authoritative surveillance tracks are consumed by ATM services.

```text
                    AUTHORITATIVE TRACKS
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
          ▼                 ▼                 ▼
   Conflict Detection   Trajectory        Airspace
                        Prediction         Processing
          │                 │                 │
          └─────────────────┼─────────────────┘
                            ▼
                       ATM STATE
```

ATM processing may include:

- Short-Term Conflict Alert (STCA)
- Trajectory prediction
- Separation monitoring
- Arrival sequencing
- Departure sequencing
- Airspace monitoring
- Sector management
- Flight-data correlation
- Controller decision support

---

# 7. Flight Data Processing

Flight data is maintained separately from surveillance data.

```text
Flight Plans
    │
    ▼
Flight Data Processing
    │
    ├── Flight Identity
    ├── Route
    ├── Clearance
    ├── Coordination
    ├── Departure Data
    └── Arrival Data
             │
             ▼
       Flight / Track Correlation
```

A flight and a surveillance track are related but are not the same domain object.

This distinction is important because:

```text
Flight Plan ≠ Aircraft Track
```

The system must explicitly model their relationship.

---

# 8. Airspace

Airspace information provides the spatial and operational context in which surveillance and ATM processing occur.

```text
                         AIRSPACE
                            │
          ┌─────────────────┼────────────────┐
          │                 │                │
          ▼                 ▼                ▼
       Sectors           Boundaries       Restrictions
          │                 │                │
          └─────────────────┼────────────────┘
                            ▼
                     ATM Processing
```

Airspace data may include:

- FIR boundaries
- Control areas
- Control zones
- Sectors
- Airways
- Restricted areas
- Danger areas
- Temporary restrictions
- Altitude limits

---

# 9. Conflict Detection and Alerting

Conflict detection operates on authoritative surveillance and relevant flight/airspace data.

```text
Tracks
  │
  ├───────────────┐
  │               │
  ▼               ▼
Track A         Track B
  │               │
  └───────┬───────┘
          ▼
    Conflict Engine
          │
          ▼
   Predicted Conflict
          │
          ▼
      Alert State
          │
          ▼
      Controller HMI
```

Safety-related calculations must remain independent of presentation logic.

The HMI displays an alert; it does not determine whether the conflict exists.

---

# 10. Weather

Weather is an independent information domain.

```text
METAR
TAF
Weather Sources
     │
     ▼
Weather Service
     │
     ▼
Normalized Weather Data
     │
     ├───────────────┐
     ▼               ▼
Controller HMI   ATM Services
```

Failure of a non-critical weather service should not unnecessarily terminate surveillance or controller operations.

---

# 11. Controller HMI

The controller application consumes authoritative system state.

```text
                 ATM Core
                    │
                    ▼
              Application API
                    │
       ┌────────────┼────────────┐
       │            │            │
       ▼            ▼            ▼
 Radar Scope   Flight Strips   Alerts
       │            │            │
       └────────────┼────────────┘
                    ▼
              Controller HMI
```

The HMI may be delivered through:

```text
apps/controller/
├── web/
└── desktop/
```

The web and desktop implementations share domain-independent frontend packages where appropriate.

---

# 12. Operational Applications

Ideal ATC contains multiple operational applications rather than treating the entire platform as one user interface.

```text
apps/
├── controller/
│   ├── web/
│   └── desktop/
│
├── flight-data/
│   ├── web/
│   └── desktop/
│
├── supervisor/
│   ├── web/
│   └── desktop/
│
├── weather/
│   ├── web/
│   └── desktop/
│
└── replay/
    ├── web/
    └── desktop/
```

Each application consumes the services and contracts required for its operational role.

---

# 13. Recording and Replay

Surveillance and relevant operational events must be recordable for simulation, debugging, testing, and analysis.

```text
Surveillance
     │
     ├───────────────► Live Processing
     │
     └───────────────► Recording
                              │
                              ▼
                         Recorded Data
                              │
                              ▼
                            Replay
                              │
                              ▼
                        ATM Processing
```

Replay must be capable of reproducing controlled scenarios without requiring live surveillance hardware.

This creates a foundation for:

- Development
- Regression testing
- Training
- Demonstration
- Incident analysis
- System verification

---

# 14. Simulation

Simulation is a first-class development environment.

```text
                SIMULATION
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   Aircraft      Sensors     Scenarios
   Generator     Simulator    Manager
        │           │           │
        └───────────┼───────────┘
                    ▼
              Surveillance
                    │
                    ▼
               ATM System
                    │
                    ▼
                  HMI
```

Simulation data must use the same canonical domain and protocol contracts wherever practical.

This prevents the simulation environment from becoming a completely separate implementation of the system.

---

# 15. Major System Boundaries

The major architectural boundaries are:

```text
┌───────────────────────────────────────────────────────────┐
│                     EXTERNAL WORLD                        │
│                                                           │
│ Radar │ ADS-B │ MLAT │ Flight Data │ Weather │ Simulator │
└───────────────────────────┬───────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                 ADAPTER / INTEGRATION LAYER               │
│                                                           │
│ ASTERIX │ ADS-B │ Radar │ Flight Data │ Weather Adapters │
└───────────────────────────┬───────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                     ATM DOMAIN                            │
│                                                           │
│ Surveillance │ Tracks │ Flight Data │ Airspace │ Alerts  │
└───────────────────────────┬───────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                    APPLICATION SERVICES                   │
│                                                           │
│ Controller │ Flight Data │ Supervisor │ Weather │ Replay │
└───────────────────────────┬───────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                         HMI                               │
│                                                           │
│             Web Applications │ Desktop Applications       │
└───────────────────────────────────────────────────────────┘
```

---

# 16. Architectural Principles

The following principles apply to the system.

### 16.1 External protocols are adapters

External protocols such as ASTERIX must not define the internal domain model.

### 16.2 Domain state has authoritative owners

Each operational data type has a defined subsystem responsible for its authoritative state.

### 16.3 HMI is not the source of truth

Applications display and modify authorized system state through defined interfaces.

### 16.4 Safety-related functions are isolated

Safety-related calculations must not depend on UI implementation details.

### 16.5 Simulation uses the same contracts

Simulated surveillance should enter the system through interfaces equivalent to real surveillance sources wherever practical.

### 16.6 Applications are independent consumers

Controller, supervisor, flight-data, weather, and replay applications should not directly depend on another application's internal implementation.

### 16.7 Language boundaries are implementation boundaries

Rust, Python, and TypeScript communicate through defined interfaces rather than sharing language-specific internal structures.

```text
Rust ───────┐
            │
Python ─────┼── Defined Protocol Contracts
            │
TypeScript ─┘
```

### 16.8 Web and desktop are deployment targets

Web and desktop applications should share appropriate frontend/domain packages without coupling the core ATM domain to a specific presentation technology.

---

# 17. Logical Dependency Direction

The preferred dependency direction is:

```text
External Systems
       │
       ▼
   Adapters
       │
       ▼
    Domain
       │
       ▼
   Processing
       │
       ▼
Application Services
       │
       ▼
     HMI
```

Dependencies should not normally flow in the opposite direction.

For example:

```text
HMI ──X──> Track Engine internals
HMI ──X──> Radar adapter
Weather ──X──> Controller internals
ASTERIX ──X──> UI
```

Instead:

```text
ASTERIX
   ↓
Adapter
   ↓
Domain
   ↓
Processing
   ↓
Contract
   ↓
Application
   ↓
HMI
```

---

# 18. Implementation Mapping

The logical architecture maps onto the repository as follows:

```text
services/core-rust/
    Surveillance
    Track Processing
    Conflict Detection
    Recording
    Replay

services/python/
    Flight Data
    Trajectory
    Weather
    Sequencing
    Analytics

proto/
    Inter-service contracts

packages/
    Shared frontend/domain functionality

apps/
    Operational applications

simulation/
    Synthetic surveillance and operational scenarios

tests/
    Verification and interoperability

docs/
    Architecture, standards, requirements and engineering rules
```

The repository structure is an implementation of the architecture; it is not the architecture itself.
