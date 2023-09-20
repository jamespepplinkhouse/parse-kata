# parse-kata

## Overview

This repo is my playground for implementing many solutions for a given CPU & IO bound problem, using different programming languages and libraries.

## Why do this?

- To learn many programming languages
- To maintain and upgrade the code over time, as new enhancements become available ("live with the tools")
- To focus on efficiency and performance
- To explore performance, efficiency, concurrency patterns and implementations
- To explore debugging and performance profiling tools for each language
- For fun!

## Project Goals

- Prefer the idiomatic style of each language (this is part of the learning)
- Compute the result as fast as possible
- Use Unit Tests
- Improve each solution over time by profiling and optimisation
- Multiple implementations using the same programming language is encouraged
- Capture performance metrics

## The problem

Given a local filesystem copy of the ["works dump"](https://openlibrary.org/developers/dumps) data set (~2.0GB compressed) from [openlibrary.org](https://openlibrary.org/) in TSV format, perform the following actions as quickly as possible (TBC):

- Parse the file, which is TSV, and has a JSON structure as the last column of each line
- Extract the title of each work
- Write the titles to a text file, with one title per line, in the exact order of the input file

Note: Sample input and correct output data is provided in the `samples/` directory.

### Notes & ideas

- At the time of writing, the `ol_dump_works_2021-11-30.txt` file contains 24,010,896 lines (13GB)
- Solutions should not attempt to load the whole input or output in memory (use streams)!
- Try a bash solution as a starting reference point (e.g. sed)
- There are unicode characters, those should be converted to UTF-8
- There are data quality issues, like leading spaces, or single quotes around titles, but for this exercise I'll just keep it raw; fixing that stuff is busy work that's not important

## Solutions

Solutions are located in the `solutions` directory using the following naming convention:
`solutions/{lang}-{single|multi}-{lines|chunks}-{json|custom}`

- `lang`: the main programming language or technology
- `single`: single threaded solution
- `multi`: multi-threaded solution
- `chunks`: the input file is read in chunks of bytes/string
- `lines`: the input file is read in lines of string
- `custom`: the search and/or extraction algorithm is custom (usually better performance)
- `json`: the extraction algorithm uses a JSON parser

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

### Example line with multiple title keys

```json
{
  "description": {
    "type": "/type/text",
    "value": "So much has how been said and written about the life and career of Michael Jackson that it has become almost impossible to disentangle the man from the myth. This book is the fruit of over 30 years of research and hundreds of exclusive interviews with a remarkable level of access to the very closest circles of the Jackson family - including Michael himself. Cutting through tabloid rumours, J. Randy Taraborrelli traces the real story behind Michael Jackson, from his drilling as a child star through the blooming of his talent to his ever-changing personal appearance and bizarre publicity stunts. This major biography includes the behind-the-scenes story to many of the landmarks in Jackson's life: his legal and commercial battles, his marriages to Lisa Marie Presley and Debbie Rowe, his passions and addictions, his children. Objective and revealing, it carries the hallmarks of all of Taraborrelli's best-sellers: impeccable research, brilliant storytelling and definitive documentation."
  },
  "subtitle": "The Magic and the Madness",
  "title": "Michael Jackson",
  "subject_places": ["United States"],
  "subjects": [
    "Rock musicians",
    "Biography",
    "Biography & Autobiography",
    "Nonfiction",
    "Jackson, michael, 1958-2009",
    "Rock musicians, biography",
    "New York Times reviewed"
  ],
  "subject_people": ["Michael Jackson (1958-)", "Michael Jackson (1958-2009)"],
  "key": "/works/OL100260W",
  "authors": [
    {
      "type": {
        "key": "/type/author_role"
      },
      "author": {
        "key": "/authors/OL32274A"
      }
    }
  ],
  "type": {
    "key": "/type/work"
  },
  "links": [
    {
      "url": "http://www.nytimes.com/1991/06/16/books/in-short-nonfiction-812891.html",
      "title": "New York Times review",
      "type": {
        "key": "/type/link"
      }
    }
  ],
  "covers": [11679779],
  "latest_revision": 10,
  "revision": 10,
  "created": {
    "type": "/type/datetime",
    "value": "2009-10-17T17:59:12.548417"
  },
  "last_modified": {
    "type": "/type/datetime",
    "value": "2021-08-17T18:47:00.471963"
  }
}
```

## Learnings!

There are a few basic things that apply to every language:

- Read the file in bytes, string conversion is too expensive
- Skip as many bytes as possible, checking each byte or window takes far too long
- The [Boyer-Moore algorithm](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm) is pretty good, some languages have that built into the base library as a strategy
- A custom algorithm that is fitted to the data is the best (can skip the most bytes)

### File structure notes

1. We need to process the file line by line, as some JSON blobs have multiple (nested) `title` fields
1. The JSON starts at the 4th tab character on each line, which is a minimum of 50 characters from the start of the line, meaning we can skip reading those characters if we know where the start of the line is
1. The titles often have JSON Unicode encoded characters in them
1. The minimum length of the JSON blobs is 260, meaning once we find a title on each line, we are able to skip some more bytes

### Suggested algorithm

1. Store the position of the start of the line
1. Skip 50
1. Store the index of the opening curly brace
1. Find the first title (TBC - need analysis)
1. Extract, decode, and store/output the title value
1. Skip 260 from the start of the JSON opening curly brace
1. Repeat!
