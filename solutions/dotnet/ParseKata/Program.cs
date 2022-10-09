namespace ParseKata;

class Program
{
  private const int MAX_BUFFER = 1048576;

  static int Main(string[] args)
  {
    var options = CommandLine.Parser.Default.ParseArguments<Options>(args);
    if (options.Errors.Count() > 0)
      return 1;

    if (options.Value.Fast)
    {
      Console.WriteLine("Using the custom parser");
      new CustomParser(options.Value).Parse();
    }
    else
    {
      Console.WriteLine("Using the JSON parser");
      new JsonParser(options.Value).Parse();
    }

    return 0;
  }
}