const int MAX_BUFFER = 1048576;

if (args.Length < 1)
  throw new Exception("First parameter (inputFilePath) not supplied");

if (args.Length < 2)
  throw new Exception("Second parameter (outputFilePath) not supplied");

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
    var titles = CustomParser.ExtractTitles(buffer);
    outputFile.Write(titles);
  }
}