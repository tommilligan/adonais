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
