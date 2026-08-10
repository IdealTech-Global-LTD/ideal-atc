docs/architecture/components.md

# Ideal ATC Component Architecture

## 1. Purpose

## 2. Component Model

## 3. Surveillance Components

### 3.1 Radar Adapters

### 3.2 ADS-B Ingestion

### 3.3 ASTERIX Processing

### 3.4 Observation Normalization

### 3.5 Track Engine

### 3.6 Track Fusion

## 4. ATM Components

### 4.1 Conflict Detection

### 4.2 Airspace Management

### 4.3 Trajectory Processing

### 4.4 Sequencing

## 5. Flight Data Components

### 5.1 Flight Plan Management

### 5.2 Flight/Track Correlation

### 5.3 Coordination

## 6. Information Services

### 6.1 Weather

### 6.2 NOTAM / Aeronautical Information

## 7. Operational Services

### 7.1 Recording

### 7.2 Replay

### 7.3 Configuration

### 7.4 Audit

### 7.5 Monitoring

## 8. Applications

### 8.1 Controller

### 8.2 Flight Data

### 8.3 Supervisor

### 8.4 Weather

### 8.5 Replay

## 9. Component Ownership

## 10. Component Dependencies

## 11. Architectural Rules

                   Surveillance
                        │
        ┌───────────────┼────────────────┐
        ▼               ▼                ▼
     Ingest          Tracking          Fusion
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 Authoritative
                     Tracks
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
        STCA        Trajectory     Airspace
