package main

import (
	"bytes"
)

func ParseLineRaw(line []byte) (title []byte) {
	_, after, found := bytes.Cut(line, []byte("\"title\": \""))

	if !found {
		return nil
	}

	before, _, found := bytes.Cut(after, []byte("\""))

	if !found {
		return nil
	}

	return before
}
