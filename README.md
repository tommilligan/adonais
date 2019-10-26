# adonais

[![Build Status](https://cloud.drone.io/api/badges/tommilligan/adonais/status.svg)](https://cloud.drone.io/tommilligan/adonais)

## Aims

- pull schedule data from keats.kcl.ac.uk
- apply filters based on user preferences
- push to calender integration of choice
- repeat periodically

## keats.kcl.ac.uk

The raw data is available from https://lsm-education.kcl.ac.uk/apicommonstring/api/values/Mod-Module.5MBBSStage2

This is an unsecured endpoint that responds to a plain GET with JSON format data.
Schema appears to be constant, with no nesting.
Keys are always present, missing data is represented by `null`s.

## Update architecture

- poll endpoint for data. Gives unfiltered data for whole year
- transform all data to internal `Event` structure
- filter by user preferences (group) into final event list for a user
- update calendar properties (name) by user preferences
- update calendar events
  - list ids of events from user's calendar from now to all future events
  - filter event objects by start time from now to a fixed periond (3 months?)
  - comparing ids from new and existing events:
    - if existing but not new, delete event
    - if new but not existing, create event
    - if new and existing, update event
  - send all events to the google API as a bulk update

## Preferences

Group 253
Calendar name: GKT Year 2

## Parsers

Unexpectedly, one of the more challenging parts was parsing the `G` field from the KEATS API.

This corresponds to `groups` an event is for. The following are all valid examples:

- ``: empty string implies all groups, `[200, 201, ..., 299]`
- `200`: a single group, `[200]`
- `200, 210 220`: several single groups, `[200, 210, 220]`
  - delimiter may be spaces, comma or a combination of both
- `210 - 212, 217-218`: a range of groups (inclusive), `[210, 211, 212, 217, 218]`
  - delimiter dash, with optional spaces
- `3 7-9, 10`: combinations of the above, `[3, 7, 8, 9, 10]`

This turns out to be too complex for regex parsing, so...

### nom

- fairly verbose. Many function definitions
- small units of grammar are easily testable
- composites nicely together
- mildly confusing documentation at first glance

### pest

- compact grammar
- macro based
- parts are not as easily testable
- lots of unwrpping due to grammar structures not testable by the compiler
- smaller `Pair` units are based on token sequences, not rust structs, making parsing less readable
- cute logo
