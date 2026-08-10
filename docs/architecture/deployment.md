Developer PC
│
├── Rust services
├── Python services
├── Web applications
├── Desktop applications
├── PostgreSQL
└── Simulation

Simulation Network
│
├── Traffic Generator
├── Surveillance Simulator
├── ATM Core
├── Database
├── Recording
└── HMI

                   SURVEILLANCE SOURCES
                           │
                  ┌────────┴────────┐
                  ▼                 ▼
             Primary Path       Backup Path
                  │                 │
                  ▼                 ▼
             Processing A      Processing B
                  │                 │
                  └────────┬────────┘
                           ▼
                       ATM Core
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
        Controller      Supervisor     Recording

# Ideal ATC Deployment Architecture

## 1. Purpose

## 2. Deployment Principles

## 3. Development Deployment

## 4. Simulation Deployment

## 5. Test Deployment

## 6. Production Deployment

## 7. Compute Nodes

## 8. Network Architecture

## 9. Process Placement

## 10. Storage

## 11. Database Deployment

## 12. Redundancy

## 13. High Availability

## 14. Backup and Recovery

## 15. Monitoring

## 16. Security Boundaries

## 17. Configuration Management

## 18. Deployment Profiles

Node A
├── surveillance-ingest
├── track-engine
└── STCA

Node B
├── flight-data
├── trajectory
└── sequencing

Node C
├── PostgreSQL
├── recording
└── operational storage

Node D
└── Controller HMI
