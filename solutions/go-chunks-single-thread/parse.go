package main

import (
	"bytes"

	"github.com/valyala/fastjson"
)

func ParseLineJson(line []byte) (title []byte) {
	jsonStartIndex := bytes.Index(line, []byte("{"))
	if jsonStartIndex == -1 {
		// There was no title field in the line
		return nil
	}

	return fastjson.GetBytes(line[jsonStartIndex:], "title")
}

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
