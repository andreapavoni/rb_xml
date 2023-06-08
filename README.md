# rb_xml: Rekordbox XML utils

A simple, light and portable command line utility to read and manipulate Rekordbox XML database files and help in common tasks which can't be done quickly through Rekordbox software.

## Features

The following list describes the status of planned and implemented features:

### Analysis

These are diagnostic tools that will just read the database and display the results.

- [x] Check missing files between Rekordbox database and the actual paths on filesystem
- [x] Check duplicated files inside a specified folder path and the ones already indexed in Rekordbox database
- [x] Check relocated files (when a file is registered to be into a path but is found elsewhere)
- [x] Check files not imported into Rekordbox database but present on filesystem

### Utilities

These tools will help you in repairing and updating the database:

- [ ] Repairing
  - [ ] Remove duplicated files
    - [ ] Interactive one-by-one
    - [ ] Bulk
  - [ ] Update paths to relocated files
    - [ ] Interactive one-by-one
    - [ ] Bulk
- [ ] Library management
  - [ ] List a summary of the files in the database
  - [ ] List a summary of the playlists in the database
  - [ ] Suggest tracks related to a specific one based on a combination of BPM, Camelot key, rating or dates range
    - [ ] Create a playlist from the results

**NOTE:** Changes to the Rekordbox XML database are written to a new XML file so that the original one can be restored.

## Installation

As of june 8th 2023, there isn't anything usable yet and it's heavily work in progress. Binaries for common platforms will be available as soon as there's some kind of command line interface.

---

©2023 a [pavonz](https://pavonz.com) joint.
