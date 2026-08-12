## `conflict-detection.md`

````markdown
# Conflict Detection Domain

## Purpose

Defines the conflict-detection concepts used by `ideal-atc`.

## Scope

This domain covers the identification and representation of potential conflicts between aircraft or other relevant traffic.

## Core Concepts

### Conflict

A condition in which defined traffic states meet criteria requiring attention from the system.

The exact criteria are to be defined.

### Conflict Detection

The process of evaluating traffic information against defined conflict criteria.

### Alert

A system-generated indication that a conflict condition has been detected.

### STCA

Short-Term Conflict Alert is a conflict-alerting function within the system architecture.

The exact algorithm, parameters, thresholds, and operational rules are not defined by this document.

## Conceptual Flow

```text
Track State
    │
    ▼
Conflict Detection
    │
    ├── No Conflict
    │
    └── Conflict
           │
           ▼
         Alert
```
````
