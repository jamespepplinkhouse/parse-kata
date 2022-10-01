class ParseKata
{
  private const int MAX_BUFFER = 1048576;

  static int Main(string[] args)
  {
    var options = CommandLine.Parser.Default.ParseArguments<Options>(args);
    if (options.Errors.Count() > 0)
      return 1;

    using (var outputFile = new FileStream(options.Value.OutputFilePath, FileMode.Create))
    using (var fs = File.Open(options.Value.InputFilePath, FileMode.Open, FileAccess.Read))
    using (var bs = new BufferedStream(fs))
    {
      var buffer = new byte[MAX_BUFFER];
      var bytesRead = 0;

      while ((bytesRead = bs.Read(buffer, 0, MAX_BUFFER)) != 0)
      {
        var titles = CustomParser.ExtractTitles(buffer);
        outputFile.Write(titles);
      }
    }

    return 0;
  }
}