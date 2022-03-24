package main

import (
	"bytes"
)

func ParseChunk(chunk []byte) (titles []byte, tail []byte) {
	// newLine := []byte("\n")
	_, after, found := bytes.Cut(chunk, []byte("\"title\": \""))

	if !found {
		return make([]byte, 0), make([]byte, 0)
	}

	before, _, found := bytes.Cut(after, []byte("\""))

	if !found {
		return make([]byte, 0), make([]byte, 0)
	}

	return before, make([]byte, 0)
}
