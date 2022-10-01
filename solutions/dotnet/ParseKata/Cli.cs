using CommandLine;

class Options
{
  [Option('i', "inputFilePath", Required = true, HelpText = "Input file path")]
  public string InputFilePath { get; set; }

  [Option('o', "outputFilePath", Required = true, HelpText = "Output file path")]
  public string OutputFilePath { get; set; }

  [Option('f', "fast",
    Default = false,
    HelpText = "Use the fast custom parser implementation")]
  public bool Fast { get; set; }
}