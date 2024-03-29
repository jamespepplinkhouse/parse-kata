namespace ParseKata;
using CommandLine;

public class Options
{
  public Options()
  {
    InputFilePath = "";
    OutputFilePath = "";
  }

  [Option('i', "inputFilePath", Required = true, HelpText = "Input file path")]
  public string InputFilePath { get; set; }

  [Option('o', "outputFilePath", Required = true, HelpText = "Output file path")]
  public string OutputFilePath { get; set; }
}