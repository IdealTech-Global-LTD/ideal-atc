## `surveillance.md`

````markdown
# Surveillance Domain

## Purpose

Defines the surveillance concepts used by `ideal-atc`.

## Scope

The surveillance domain covers the acquisition and representation of information about aircraft or other relevant targets from surveillance sources.

## Core Concepts

### Surveillance Source

A source that provides surveillance information to the system.

The supported source types are to be defined.

### Observation

Information received from a surveillance source concerning a target.

### Surveillance Data

The data produced or received by a surveillance source before or during processing into system-level track information.

### Track Association

The process of associating surveillance observations with an existing system track.

The exact algorithm and rules are to be defined.

## Processing

The intended conceptual flow is:

```text
Surveillance Source
        │
        ▼
     Observation
        │
        ▼
     Processing
        │
        ▼
      Track
```
````
