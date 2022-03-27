package main

import (
	"bytes"
)

func ParseChunk(chunk []byte) (titles []byte, tail []byte) {
	chunkLength := len(chunk)
	target := []byte("\"title\": \"")
	cursor := 0

	// fmt.Println("ParseChunk", len(chunk))
	// fmt.Println(string(chunk[0:100]))

	for cursor < chunkLength {
		nextTargetIndex := bytes.Index(chunk[cursor:], target)
		if nextTargetIndex < 0 {
			// There are no more titles in the rest of the chunk
			break
		} else {
			titleStart := nextTargetIndex + 10
			titleEndIndex := bytes.Index(chunk[cursor+titleStart:], []byte("\""))

			if titleEndIndex < 0 {
				// The end of the title is not in this chunk
				tail = chunk[cursor+titleStart:]
				break
			}

			titleEnd := titleStart + titleEndIndex
			titles = append(titles, chunk[cursor+titleStart:cursor+titleEnd]...)
			titles = append(titles, []byte("\n")...)
			cursor = cursor + titleEnd
		}
	}

	return titles, tail
}
