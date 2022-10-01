class ParseKata
{
  private const int MAX_BUFFER = 1048576;

  static int Main(string[] args)
  {
    var options = CommandLine.Parser.Default.ParseArguments<Options>(args);
    if (options.Errors.Count() > 0)
      return 1;

    if (options.Value.Fast)
      new CustomParser(options).Parse();
    else
      new JsonParser(options).Parse();

    return 0;
  }
}