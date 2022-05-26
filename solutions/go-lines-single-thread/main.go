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

	// TODO: be more specific, or use a library
	args := os.Args[1:]
	if len(args) != 2 {
		log.Fatal("Incorrect arguments supplied")
	}

	inputFile := args[0]

	fmt.Println("Reading from: ", inputFile)

	// Input file setup
	input, err := os.Open(inputFile)
	if err != nil {
		log.Fatalf("Failed reading input file: %s", err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	maxCapacity := 1024 * 1024
	buf := make([]byte, maxCapacity)
	scanner.Buffer(buf, maxCapacity)

	counter := 0

	for scanner.Scan() {
		counter++
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	log.Println(counter)
}
