# ADS-B Ingest

Receives ADS-B surveillance data and converts it into `Observation` objects.

## Responsibilities

* Listen for incoming ADS-B messages
* Parse raw frames
* Decode aircraft data
* Produce `Observation` models

## Not responsible for

* Aircraft tracking
* Conflict detection
* WebSockets
* Database storage
* User interface

## Data flow

Ground Receiver → ADS-B Ingest → Observation → Track Engine
