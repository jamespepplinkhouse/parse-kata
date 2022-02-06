package main

import (
	"bytes"

	"github.com/valyala/fastjson"
)

func ParseLineJson(line []byte) (title []byte) {
	jsonStartIndex := bytes.Index(line, []byte("{"))
	return fastjson.GetBytes(line[jsonStartIndex:], "title")
}

func ParseLineRaw(line []byte) (title []byte) {
	titleStart := bytes.Index(line, []byte("\"title\": \"")) + 10
	titleEnd := titleStart + bytes.Index(line[titleStart:], []byte("\""))
	return line[titleStart:titleEnd]
}
