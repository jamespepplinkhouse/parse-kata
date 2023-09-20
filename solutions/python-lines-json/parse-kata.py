#!/usr/bin/env python3

import argparse
import json
import os

def process_file(inputFilePath, outputFilePath):
    if not os.path.exists(inputFilePath):
        raise FileNotFoundError(f"Input file '{inputFilePath}' does not exist.")
    
    with open(inputFilePath, 'r') as infile, open(outputFilePath, 'w') as outfile:
        for line in infile:
            try:
                start_idx = line.index('{')
            except ValueError:
                print("No JSON object found in a line. Skipping...")
                continue

            json_str = line[start_idx:].strip()
            try:
                json_obj = json.loads(json_str)
            except json.JSONDecodeError:
                print("Invalid JSON object found. Skipping...")
                continue

            title = json_obj.get('title', 'N/A')
            outfile.write(title + '\n')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Process TSV file with JSON in the last column.')
    parser.add_argument('inputFilePath', type=str, help='Path to the input file.')
    parser.add_argument('outputFilePath', type=str, help='Path to write the output file.')
    
    args = parser.parse_args()
    
    try:
        process_file(args.inputFilePath, args.outputFilePath)
        print(f"Processed file. Titles written to '{args.outputFilePath}'.")
    except Exception as e:
        print(f"An error occurred: {e}")
