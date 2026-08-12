# Airspace Domain

## Purpose

Defines the airspace concepts used by `ideal-atc`.

This document describes the domain concepts only. It does not define implementation details, communication protocols, or operational procedures.

## Scope

The airspace domain covers the representation and use of airspace information by the system.

## Core Concepts

### Airspace

An airspace entity represented by the system.

The exact airspace classification, geometry model, and operational attributes are to be defined as the system develops.

### Sector

A defined portion of airspace that may be associated with an operational responsibility.

The exact representation and assignment rules are to be defined.

### Boundary

Defines the spatial extent of an airspace entity.

The geometry representation is to be defined by the system's geographic model.

## Domain Relationships

```text
Airspace
   ├── Sector
   └── Boundary

Airspace information
   └── may be consumed by surveillance,
       conflict detection, flight-data,
       and controller applications
```
