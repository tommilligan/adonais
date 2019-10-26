# Google API - Scopes justification

## Overview

This app pulls data from a public endpoint, which hosts event data in a custom format. It then transforms it to the Google Calendar Event format, and pushes it into a user's calendar.

## Scopes

### `email`

User's email address is used by Firebase to generate a unique `uid` for the user.

This `uid` is required to store per-user data in Firebase.

### `profile`

User's basic profile information is used to customise the app, e.g. profile picture, human-readable name.

### `https://www.googleapis.com/auth/calendar`

The app will create a new calendar for storing event data in, to avoid polluting the user's other calendars.
The app does not read data from any user calendar it has not created.

### `https://www.googleapis.com/auth/calendar.events`

The app will:

- list
- insert
- delete

events from the user's calendar it manages.

The app does not read data from any user calendar it has not created.
