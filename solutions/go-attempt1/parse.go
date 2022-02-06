package main

import (
	"bytes"

	"github.com/valyala/fastjson"
)

// 24 seconds
func ParseLineJson(line []byte) (title string) {
	jsonStartIndex := bytes.Index(line, []byte("{"))
	return fastjson.GetString(line[jsonStartIndex:], "title")
}

// 7.35 seconds
func ParseLineRaw(line []byte) (title string) {
	titleStart := bytes.Index(line, []byte("\"title\": \"")) + 10
	titleEnd := titleStart + bytes.Index(line[titleStart:], []byte("\""))
	return string(line[titleStart:titleEnd])
}
