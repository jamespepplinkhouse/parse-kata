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
- Capture performance metrics ([hyperfine](https://github.com/sharkdp/hyperfine))
  - CPU usage
  - Memory usage
  - Run time

## The problem

Given a local filesystem copy of the ["works dump"](https://openlibrary.org/developers/dumps) data set (~2.0GB compressed) from [openlibrary.org](https://openlibrary.org/) in TSV format, perform the following actions as quickly as possible (TBC):

- Parse the file (TSV, JSON)
- Transform the data (i.e. MapReduce) and output the following as separate TSV files:
  - A sorted unique list of Authors
  - A sorted unique list of Titles
  - ...

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

#### Backreferences

- editions from /type/edition.works

### /type/author_role

#### Properties

- author of type /type/author
- role of type /type/string
- as of type /type/string
