package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"

	"github.com/pkg/profile"
)

const chunkSize = 134217728 // 128MB

func main() {
	defer profile.Start(profile.ProfilePath(".")).Stop()

	args := os.Args[1:]
	if len(args) != 2 {
		log.Fatal("First arg is input file; second arg is output file")
	}

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
	var lastTail = make([]byte, 0)

	for {
		bytesRead, err := input.Read(inputBuffer)
		if err != nil && err != io.EOF {
			log.Fatal(err)
		}

		chunk := append(lastTail, inputBuffer[:bytesRead]...)
		titles, _ := ParseChunk(chunk)
		// lastTail = tail

		if len(titles) > 0 {
			_, _ = datawriter.Write(titles)
		}

		if err == io.EOF {
			break
		}
	}
}
