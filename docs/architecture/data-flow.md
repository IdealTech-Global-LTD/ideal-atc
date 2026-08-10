# Ideal ATC Data Flow

## 1. Purpose

## 2. Data Flow Principles

## 3. Surveillance Data Flow

### 3.1 Radar

### 3.2 ADS-B

### 3.3 MLAT

### 3.4 ASTERIX

## 4. Observation Normalization

## 5. Track Processing

## 6. Track Fusion

## 7. Flight Data Flow

## 8. Conflict Detection Flow

## 9. Weather Data Flow

## 10. HMI Data Flow

## 11. Recording Flow

## 12. Replay Flow

## 13. Error and Degraded Data Flow

## 14. Data Lifecycle

## 15. Data Ownership

Radar / ADS-B
│
▼
Input Adapter
│
▼
Raw Observation
│
▼
Validation
│
▼
Normalization
│
▼
Track Association
│
▼
Track Update
│
▼
Track Fusion
│
▼
Authoritative Track
│
├──────────────► STCA
│
├──────────────► Trajectory
│
└──────────────► Controller API
│
▼
Radar Scope
