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
	titleStart := bytes.Split(line, []byte("\"title\": \""))

	if len(titleStart) <= 1 {
		return nil
	}

	titleLength := bytes.Index(titleStart[1], []byte("\""))

	if titleLength == -1 {
		return nil
	}

	return titleStart[1][:titleLength]

}
