package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
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

	// Output output setup
	output, err := os.OpenFile(outputFile, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		log.Fatalf("Failed creating output file: %s", err)
	}
	datawriter := bufio.NewWriter(output)
	defer datawriter.Flush()
	defer output.Close()

	scanner := bufio.NewScanner(input)
	maxCapacity := 1024 * 1024
	buf := make([]byte, maxCapacity)
	scanner.Buffer(buf, maxCapacity)

	for scanner.Scan() {
		// fmt.Println(ParseLine(scanner.Bytes()))
		_, _ = datawriter.WriteString(ParseLine(scanner.Bytes()) + "\n")
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
