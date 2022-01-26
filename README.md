# parse-kata

## Overview

This repo is my playground for implementing many solutions for a given CPU & IO bound problem, using different programming languages and libraries.

## Why do this?

- To learn many programming languages
- To maintain and upgrade the code over time, as new enhancements become available ("live with the tools")
- To focus on efficiency and performance
- To explore concurrency patterns and implementations
- To explore debugging and performance profiling tools for each language
- For fun!

## Project Goals

- Prefer the idiomatic style of each language (this is part of the learning)
- Get the results as fast as possible
- Improve each solution over time by profiling and optimisation
- Multiple implementations using the same programming language is encouraged
- Capture performance metrics ([hyperfine](https://github.com/sharkdp/hyperfine) | `time` command ??)

## The problem

Given a local filesystem copy of the ["works dump"](https://openlibrary.org/developers/dumps) data set (~2.0GB compressed) from [openlibrary.org](https://openlibrary.org/) in TSV format, perform the following actions as quickly as possible (TBC):

- Parse the file (TSV, JSON)
- Extract the title of each work
- Sort A-Z
- Write to a text file, with one title per line

### Notes & ideas

- At the time of writing, the `ol_dump_works_2021-11-30.txt` file contains 24,010,896 lines (13GB)
- Solutions should not attempt to load the whole input or output in memory (use streams)!
- Try a bash solution as a starting reference point (e.g. sed & sort)
- There are data quality issues, like leading spaces, or single quotes around titles, but for this exercise I'll just keep it raw; fixing that stuff is busy work that's not important

### Approach 1

- [Main thread] Establish a worker pool, matching the CPUs in the system
- [Main thread] Read the input file stream in chunks
- [Main thread] Queue the chunks as jobs for processing
- [Worker thread] Extract the title values
- [Worker thread] Sort into letter buckets (i.e. all of the A records together)
- [Worker thread] Return the results to [Main thread]
- [Main thread] Append results to temporary files, one for each letter
- [Main thread] Queue temp files as jobs for sorting
- [Worker thread] Sort a temp file
- [Main thread] Concatenate for final output

## Data format

All of the dumps are formatted as tab separated files with the following columns:

- type - type of record (/type/edition, /type/work etc.)
- key - unique key of the record. (/books/OL1M etc.)
- revision - revision number of the record
- last_modified - last modified timestamp
- JSON - the complete record in JSON format

### /type/work

REF: https://openlibrary.org/type/work

#### Properties

- title of type /type/string
- subtitle of type /type/string
- authors[] of type /type/author_role
- translated_titles[] of type /type/translated_string
- subjects[] of type /type/string
- subject_places[] of type /type/string
- subject_times[] of type /type/string
- subject_people[] of type /type/string
- description of type /type/text
- dewey_number[] of type /type/string
- lc_classifications[] of type /type/string
- first_sentence of type /type/text
- original_languages[] of type /type/language
- other_titles[] of type /type/string
- first_publish_date of type /type/string
- links[] of type /type/link
- notes of type /type/text
- cover_edition of type /type/edition
- covers[] of type /type/int

### Sample Data

```
/type/work	/works/OL10000049W	3	2010-04-28T06:54:19.472104	{"title": "Le crime organise/quatre films exemplaires", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140091], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000049W", "authors": [{"type": "/type/author_role", "author": {"key": "/a/OL3964979A"}}], "type": {"key": "/type/work"}, "id": 45170289, "revision": 3}
/type/work	/works/OL10000394W	3	2010-04-28T06:54:19.472104	{"title": "Portraits of Illusions", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140733], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000394W", "authors": [{"type": "/type/author_role", "author": {"key": "/a/OL3965434A"}}], "type": {"key": "/type/work"}, "id": 45170634, "revision": 3}
/type/work	/works/OL10000415W	4	2020-12-08T01:02:12.689209	{"title": "Le Berceau du monde", "covers": [3140790], "key": "/works/OL10000415W", "authors": [{"type": {"key": "/type/author_role"}, "author": {"key": "/authors/OL3965461A"}}], "type": {"key": "/type/work"}, "subjects": ["Description and travel"], "latest_revision": 4, "revision": 4, "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "last_modified": {"type": "/type/datetime", "value": "2020-12-08T01:02:12.689209"}}
/type/work	/works/OL10000427W	4	2010-07-20T14:36:28.095870	{"created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "subject_places": ["France"], "subjects": ["Muslims", "National characteristics, French", "Social integration", "Cultural assimilation", "Immigrants", "Ethnic relations", "Islam", "French National characteristics"], "latest_revision": 4, "key": "/works/OL10000427W", "title": "La fracture identitaire", "authors": [{"type": {"key": "/type/author_role"}, "author": {"key": "/authors/OL3965478A"}}], "type": {"key": "/type/work"}, "last_modified": {"type": "/type/datetime", "value": "2010-07-20T14:36:28.095870"}, "revision": 4}
/type/work	/works/OL10000471W	3	2010-04-28T06:54:19.472104	{"title": "Jamiroqua\u00ef de A \u00e0 Z", "created": {"type": "/type/datetime", "value": "2009-12-11T01:57:19.964652"}, "covers": [3140910], "last_modified": {"type": "/type/datetime", "value": "2010-04-28T06:54:19.472104"}, "latest_revision": 3, "key": "/works/OL10000471W", "authors": [{"type": "/type/author_role", "author": {"key": "/a/OL3965508A"}}], "type": {"key": "/type/work"}, "id": 45170711, "revision": 3}
```
