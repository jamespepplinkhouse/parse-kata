using ArgParse
using JSON

function load_and_extract(inputFilePath::String, outputFilePath::String)
    # Check if the input file exists
    input_file_exist = isfile(inputFilePath)
    if (!input_file_exist)
        println("Input file does not exist: ", inputFilePath)
        return
    end

    # Preparing output file
    out = open(outputFilePath, "w") 
    try
        open(inputFilePath, "r") do file
            for line in eachline(file)
                json_start_index = findfirst('{', line)
                if (isnothing(json_start_index))
                    # If no JSON document is found, continue to the next line
                    continue
                end
                json_doc = line[json_start_index:end]
                json_parsed = try
                    JSON.parse(json_doc)
                catch e
                    println("Error parsing JSON on line: ", line)
                    println("Error: ", e)
                    # If JSON parsing fails, continue to the next line
                    continue
                end
                title = get(json_parsed, "title", "")
                write(out, "$title\n")
            end
        end
    finally
        close(out)
    end
end

function main(args)
    s = ArgParseSettings()

    @add_arg_table! s begin
        "inputFilePath"
            help = "path to the input file"
            arg_type = String
        "outputFilePath"
            help = "path to write the output file"
            arg_type = String
    end

    parsed_args = parse_args(args, s)
    
    println("Starting time measurement...")
    @time load_and_extract(parsed_args["inputFilePath"], parsed_args["outputFilePath"])
    println("...End of time measurement.")
end

# Run the main function with command line arguments
main(ARGS)