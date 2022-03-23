package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"log"
	"os"
)

const chunkSize = 524288000 // 500MB

func main() {
	defer profile.Start(profile.ProfilePath(".")).Stop()

	args := os.Args[1:]
	if len(args) != 2 {
		log.Fatal("Incorrect arguments supplied")
	}
	// TODO: be more specific, or use a library

	inputFile := args[0]
	outputFile := args[1]

	fmt.Println("Reading from: ", inputFile)
	fmt.Println("Writing to:", outputFile)

	// Input file setup
	input, err := os.Open(inputFile)
	if err != nil {
		log.Fatalf("Failed reading input file: %s", err)
	}
	defer input.Close()

	// Output file setup
	output, err := os.OpenFile(outputFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Fatalf("Failed creating output file: %s", err)
	}
	datawriter := bufio.NewWriter(output)
	defer datawriter.Flush()
	defer output.Close()

	inputBuffer := make([]byte, chunkSize)
	newLine := []byte("\n")
	// previousPartialLine := make([]byte, 0)

	for {
		bytesRead, err := input.Read(inputBuffer)
		if err != nil && err != io.EOF {
			log.Fatal(err)
		}

		lines := bytes.Split(inputBuffer[:bytesRead], newLine)
		fmt.Println("Found %d lines", len(lines))
		// keep the tail

		titles := make([]byte, 0)
		for _, line := range lines {
			title := ParseLineRaw(line)

			if title != nil {
				titles = append(titles, title...)
				titles = append(titles, newLine...)
			}
		}

		_, _ = datawriter.Write(titles)

		if err == io.EOF {
			break
		}
	}
}
