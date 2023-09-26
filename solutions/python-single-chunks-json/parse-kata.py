#!/usr/bin/env python3


from json import JSONDecodeError
import pandas as pd
import time
import argparse
import json
import os

def process_file(inputFilePath: str, outputFilePath: str):
    line_count: int = 0
    error_count: int = 0
    chunk_size: int = 10000
    
    if not os.path.exists(inputFilePath):
        raise FileNotFoundError(f"Input file '{inputFilePath}' does not exist.")
    
    for chunk in pd.read_csv(inputFilePath, sep='\t', chunksize=chunk_size, header=None,
                             usecols=[4], quoting=3, dtype=str):
        for line in chunk.values:
            try:
                book = json.loads(line[0])
                line[0] = book['title']
                line_count = line_count + 1
            except Exception as e:
                print("There was an error extracting the json text.")
                print(e)
                print("Error Json text: ", line[0])                
                error_count = error_count + 1

        chunk.to_csv(outputFilePath, mode='a', header=False, index=False, index_label=None, columns=[4])

    if error_count > 0:
        print("Total json parse errors: ", error_count)

    print("Total lines parsed:", line_count)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Process TSV file with JSON in the last column.')
    parser.add_argument('inputFilePath', type=str, help='Path to the input file.')
    parser.add_argument('outputFilePath', type=str, help='Path to write the output file.')
    
    args = parser.parse_args()
    
    start_time = time.perf_counter()
##    try:
    process_file(args.inputFilePath, args.outputFilePath)
    print(f"Processed file. Titles written to '{args.outputFilePath}'.")
##    except Exception as e:
##        print(f"An error occurred: {e}")
    
    end_time = time.perf_counter()
    print('process completed in ' + str(end_time - start_time)[0:5] + ' seconds.')