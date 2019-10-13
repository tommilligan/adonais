# adonais

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
