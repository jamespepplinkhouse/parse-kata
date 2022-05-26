using System.IO;

var inputFile = args[0];
// var outputFile = args[1];

// var counter = 0;
// foreach (var line in File.ReadLines(Path.GetFullPath(inputFile)))
// {
//   counter++;
//   // Console.WriteLine(line);
// }
// Console.WriteLine($"Found {counter} lines");


const int MAX_BUFFER = 1048576; //1MB 
byte[] buffer = new byte[MAX_BUFFER];
int bytesRead;
var chunks = 0;
using (var fs = File.Open(inputFile, FileMode.Open, FileAccess.Read))
using (var bs = new BufferedStream(fs))
{
  while ((bytesRead = bs.Read(buffer, 0, MAX_BUFFER)) != 0) //reading 1mb chunks at a time
  {
    chunks++;
    //Let's create a small size file using the data. Or Pass this data for any further processing.
  }
}

var fn = () =>
{
  Console.WriteLine("Hai");
};

fn();

Console.WriteLine($"Found {chunks} chunks");

// Input file setup
// Output file setup
// parse
