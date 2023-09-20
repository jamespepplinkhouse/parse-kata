package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"github.com/pkg/profile"
)

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

	// The split function defaults to ScanLines.
	scanner := bufio.NewScanner(input)
	maxCapacity := 1024 * 1024
	buf := make([]byte, maxCapacity)
	scanner.Buffer(buf, maxCapacity)

	newLine := []byte("\n")

	for scanner.Scan() {
		_, _ = datawriter.Write(ParseLineJson(scanner.Bytes()))
		datawriter.Write(newLine)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
