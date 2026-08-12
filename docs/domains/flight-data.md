## `flight-data.md`

````markdown
# Flight Data Domain

## Purpose

Defines the flight-data concepts used by `ideal-atc`.

## Scope

The flight-data domain represents information associated with flights and their operational processing within the system.

## Core Concepts

### Flight

A flight entity represented by the system.

### Flight Plan

Information describing a planned flight.

The exact fields and lifecycle are to be defined.

### Aircraft

An aircraft associated with flight information.

Aircraft identity and association rules are to be defined.

### Callsign

An identifier associated with a flight for operational purposes.

### Clearance

A representation of an authorized instruction associated with a flight.

The exact model and scope are to be defined.

## Domain Relationships

```text
Flight
 ├── Aircraft
 ├── Flight Plan
 ├── Callsign
 └── Clearance
```
````
