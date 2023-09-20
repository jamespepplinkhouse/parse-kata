namespace ParseKata;

class Program
{
  static int Main(string[] args)
  {
    var options = CommandLine.Parser.Default.ParseArguments<Options>(args);
    if (options.Errors.Count() > 0)
      return 1;

    new JsonParser(options.Value).Parse();

    return 0;
  }
}