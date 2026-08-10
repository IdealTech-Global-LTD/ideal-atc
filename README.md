# ideal-atc

> Next-Generation Air Traffic Control & Telemetry Microservices System

`ideal-atc` is an experimental, distributed air traffic management system designed for real-time ADS-B target ingestion, flight path trajectory estimation, short-term conflict alerting (STCA), and controller radar displays.

---

## High-Level Architecture Overview

The system is structured as a collection of decoupled, domain-specific microservices communicating over high-performance IPC boundaries (gRPC & WebSockets):

---

## 🏗️ Repository Architecture

The project is structured into modular workspaces, desktop applications, and decoupled microservices communicating over high-performance IPC boundaries (gRPC & WebSockets):

````text
ideal-atc/
├── apps/              # Native Desktop Applications (Rust + Tao/Wry + WebGL)
├── services/          # Backend Microservices (Rust & Python runtimes)
├── packages/          # Shared crates & packages (spatial math, UI primitives)
├── proto/             # Centralized Protobuf schemas & gRPC contracts
├── simulation/        # ADS-B target generators & track scenario replay engines
├── tests/             # End-to-end integration tests & safety benchmarks
└── docs/              # System specifications & API documentation

---

## Teams & Service Ownership

| Domain | Responsible Service | Focus Area |
| :--- | :--- | :--- |
| **Protocol Contracts** | `proto/` | gRPC & Protobuf schemas (`.proto`) |
| **Telemetry & Core Engine** | `services/core-rust/` | ADS-B Mode-S ingestion, STCA collision algorithms, WebSocket streaming |
| **Flight Analytics & AI** | `services/analytics-python/` | 4D trajectory modeling, METAR parsers, arrival queues |
| **Controller Interfaces** | `apps/` | WebGL canvas radar display, electronic flight strip management |

---

## 🛠️ Local Development Quickstart

### Prerequisites
* **Git**
* **Rust** (Cargo stable)
* **Python** (3.11+)
* **Node.js** (v20+) or **pnpm**
* **Docker** & Docker Compose (optional for full containerized stack)

### 1. Clone the Repository
```bash
git clone [https://github.com/IdealTech-Global-LTD/ideal-atc.git](https://github.com/IdealTech-Global-LTD/ideal-atc.git)
cd ideal-atc
````
