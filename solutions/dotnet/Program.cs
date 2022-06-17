using System.Text.RegularExpressions;
using System.Text;

public class Program
{
  const int MAX_BUFFER = 1048576;

  public static void Main(string[] args)
  {
    var inputFilePath = args[0];
    var outputFilePath = args[1];
    var titleRegex = new Regex("title\": \"(.+?)\"", RegexOptions.Compiled);

    using (var outputFile = new FileStream(outputFilePath, FileMode.Create))
    using (var fs = File.Open(inputFilePath, FileMode.Open, FileAccess.Read))
    using (var bs = new BufferedStream(fs))
    {
      byte[] buffer = new byte[MAX_BUFFER];
      int bytesRead;

      while ((bytesRead = bs.Read(buffer, 0, MAX_BUFFER)) != 0)
      {
        // TODO: buffer might contain a chopped title at the end
        // TODO: Don't convert to string, scan each byte once only
        // TODO: Don't use regex, too slow!
        // TODO: After a newline, skip 50 bytes
        var content = Encoding.Default.GetString(buffer);
        var matches = titleRegex.Matches(content);
        var titles = matches.Select(x => x.Groups[1].Captures.First().Value);
        outputFile.Write(Encoding.ASCII.GetBytes(String.Join('\n', titles)));
      }
    }
  }
}


