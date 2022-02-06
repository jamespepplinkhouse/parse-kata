package main

import (
	"bytes"

	"github.com/valyala/fastjson"
)

func ParseLine(line []byte) (title string) {
	jsonStartIndex := bytes.Index(line, []byte("{"))
	return fastjson.GetString(line[jsonStartIndex:], "title")
}
