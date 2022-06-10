using System.Text.RegularExpressions;

public class Program
{
  public static void Main(string[] args)
  {
    var inputFilePath = args[0];
    var outputFilePath = args[1];
    Console.WriteLine($"inputFilePath: {inputFilePath}");
    Console.WriteLine($"outputFilePath: {outputFilePath}");

    var title = new Regex("title\": \"(.+?)\"");

    foreach (var line in File.ReadLines(inputFilePath))
    {
      var matches = title.Match(line);
      if (matches.Groups.Count > 1)
      {
        Console.WriteLine(matches.Groups[1].Captures.First());
      }
    }
  }
}