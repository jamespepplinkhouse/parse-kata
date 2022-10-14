namespace ParseKata;

class Program
{
  static int Main(string[] args)
  {
    var options = CommandLine.Parser.Default.ParseArguments<Options>(args);
    if (options.Errors.Count() > 0)
      return 1;

    Console.WriteLine("Using the {0} parser", options.Value.Fast ? "custom" : "JSON");

    if (options.Value.Fast)
      new CustomParser(options.Value).Parse();
    else
      new JsonParser(options.Value).Parse();

    return 0;
  }
}