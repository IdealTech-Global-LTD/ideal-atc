## `weather.md`

````markdown
# Weather Domain

## Purpose

Defines the weather-information concepts used by `ideal-atc`.

## Scope

The weather domain covers weather information consumed or represented by the system.

## Core Concepts

### Weather Observation

A reported observation of weather conditions.

### METAR

A standardized aviation weather observation format.

The system's supported fields and processing requirements are to be defined.

### TAF

A standardized aviation terminal forecast format.

The system's supported fields and processing requirements are to be defined.

## Conceptual Flow

```text
Weather Source
      │
      ▼
Weather Data
      │
      ├── Observation
      │
      └── Forecast
              │
              ▼
       Weather Services
              │
              ▼
        Applications
```
````
