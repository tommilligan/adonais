# adonais

[![Build Status](https://cloud.drone.io/api/badges/tommilligan/adonais/status.svg)](https://cloud.drone.io/tommilligan/adonais)

KEATS to Google Calendar integration.

## How it works

[Use it here!](https://adonais.tommilligan.net)

`adonais` takes publicly accessible data from KEATS, transforms it, then uploads it to a new calendar in your account.

- Login with your Google Account
- Give `adonais` permission to manage your calendar data
- Click the _Sync Calendar Now_ button when it turns blue
- Marvel at your new/updated calendar

**Note:** `adonais` will not update your calendar automatically.

### Your data

To use `adonais`, you must grant it access to:

- read/write **all** your calendars
- read/write **all** your events

At worst, this means `adonais` could delete or make public **all** your calendar data. There's an alternate design that's much better, which is currently in progress (see [Todo List](#todo-list)).

However, `adonais` is currently designed so that it:

- **never** deletes calendars, it only creates them
- **only** reads and writes data to calendars it has created

You **should not** edit the calendar `adonais` creates, as it will overwrite any changes on the next update.

If you think there is a bug in how `adonais` behaves, please [open a new issue](https://github.com/tommilligan/adonais/issues).

## Working notes

### Todo List

- [x] pull event data from keats.kcl.ac.uk
  - [x] setup proxy for CORS
- [x] data transformation
  - [x] deserialize from keats events
    - [x] timezone aware transforms for naive data
    - [x] parser for groups syntax
  - [x] filter events to relevant subset
  - [x] diff events to only those that need updates
  - [x] serialize to google events
- [x] push to user calendar
  - [x] create new calendar and store reference
  - [x] push events to calendar
- [ ] UI
  - [x] sync calendar now
  - [x] delete account
  - [ ] change user settings
    - [ ] group
- [ ] make calendars namespaced under service account
  - [ ] keep track of all calendars centrally
  - [ ] give user read-only access to calendar that matches their settings
- [ ] batch service
  - [ ] fetch all users & credentials from store
  - [ ] run sync for a user without interactive login
  - [ ] schedule to run periodically

### Update architecture

- Fetch data from KEATS. Gives unfiltered data for whole year.
  - Transform data and generate id from `Event` hash
  - Filter by user preferences (group) into final event list for a user
- Fetch data from GCal. Gives existing event ids from now onwards.
- Compare ids from new and existing events:
  - if existing but not new, delete event
  - if new but not existing, create event
  - if new and existing, no action
- Send all updates to the Google API in bulk

### keats.kcl.ac.uk

The raw data is available from https://lsm-education.kcl.ac.uk/apicommonstring/api/values/Mod-Module.5MBBSStage2

This is an unsecured endpoint that responds to a plain GET with JSON format data (amazing!)
Schema appears to be constant, with no nesting.
Keys are always present, missing data is represented by `null`s.

### Preferences

Initial preferences to hardcode:

- Group: 253
- Calendar: GKT Year 2

### Parsers

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

#### nom

- fairly verbose. Many function definitions
- small units of grammar are easily testable
- composites nicely together
- mildly confusing documentation at first glance

#### pest

- compact grammar
- macro based
- parts are not as easily testable
- lots of unwrpping due to grammar structures not testable by the compiler
- smaller `Pair` units are based on token sequences, not rust structs, making parsing less readable
- cute logo
