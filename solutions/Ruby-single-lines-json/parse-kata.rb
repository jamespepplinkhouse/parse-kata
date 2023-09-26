require 'json'

def parse_tsv_file(input_file_path, output_file_path)
  # Check if input file exists
  unless File.exist?(input_file_path)
    raise ArgumentError, "Input file '#{input_file_path}' not found."
  end

  start_time = Time.now

  # Open the input and output files
  File.open(input_file_path, 'r') do |input_file|
    File.open(output_file_path, 'w') do |output_file|
      input_file.each_line do |line|
        # Split the TSV line into columns
        columns = line.split("\t")

        # Extract the JSON document from the last column
        json_doc = columns.last.strip

        # Parse the JSON document
        json_data = JSON.parse(json_doc)

        # Extract the title from the JSON data
        title = json_data['title']

        # Write the title to the output file
        output_file.puts(title)
      end
    end
  end

  elapsed_time = Time.now - start_time

  puts "Output file '#{output_file_path}' generated successfully."
  puts "Elapsed time: #{elapsed_time} seconds."
rescue JSON::ParserError
  raise ArgumentError, "Invalid JSON document found in input file."
rescue => e
  raise e
end

# Check if the required input file and output file paths are provided as command-line arguments
if ARGV.length < 2
  puts "Usage: ruby parse-kata.rb <inputFilePath> <outputFilePath>"
else
  # Extract the input file path and output file path from command-line arguments
  input_file_path = ARGV[0]
  output_file_path = ARGV[1]

  # Call the function to parse the TSV file and generate the output file
  parse_tsv_file(input_file_path, output_file_path)
end