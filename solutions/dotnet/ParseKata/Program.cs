public class Program
{
  const int MAX_BUFFER = 1048576;

  public static void Main(string[] args)
  {
    var inputFilePath = args[0];
    var outputFilePath = args[1];

    using (var outputFile = new FileStream(outputFilePath, FileMode.Create))
    using (var fs = File.Open(inputFilePath, FileMode.Open, FileAccess.Read))
    using (var bs = new BufferedStream(fs))
    {
      var buffer = new byte[MAX_BUFFER];
      var bytesRead = 0;

      while ((bytesRead = bs.Read(buffer, 0, MAX_BUFFER)) != 0)
      {
        var titles = Logic.ExtractTitles(buffer);
        outputFile.Write(titles);
      }
    }
  }
}



