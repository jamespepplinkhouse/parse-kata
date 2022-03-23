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
	titleKeyStartIndex := bytes.Index(line, []byte("\"title\": \""))

	if titleKeyStartIndex == -1 {
		// There was no title field in the line
		return nil
	}

	// The title value starts at +10 from the leading quote of the json title key
	titleValueStartIndex := titleKeyStartIndex + 10

	if titleValueStartIndex > len(line) {
		return nil
	}

	titleLength := bytes.Index(line[titleValueStartIndex:], []byte("\""))

	if titleLength == -1 {
		return nil
	}

	return line[titleValueStartIndex : titleValueStartIndex+titleLength]
}
