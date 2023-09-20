package main

import (
	"bytes"

	"github.com/valyala/fastjson"
)

func ParseLineJson(line []byte) (title []byte) {
	jsonStartIndex := bytes.Index(line, []byte("{"))
	return fastjson.GetBytes(line[jsonStartIndex:], "title")
}
