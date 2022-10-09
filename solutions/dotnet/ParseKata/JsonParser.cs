namespace ParseKata;
using System.Text.Json;
using System.Text.Json.Serialization;

class JsonParser
{
  private Options _options { get; set; }
  public JsonParser(Options options)
  {
    this._options = options;
  }

  public void Parse()
  {
    using (StreamWriter writer = new StreamWriter(this._options.OutputFilePath))
    {
      foreach (var line in File.ReadLines(this._options.InputFilePath))
      {
        // Find the index of the fourth tab character
        var jsonStartIndex = 0;
        for (int i = 0; i < 4; i++)
        {
          var newIndex = line.IndexOf('\t', jsonStartIndex) + 1;
          if (newIndex == -1)
          {
            jsonStartIndex = 0;
            break;
          }
          jsonStartIndex = newIndex;
        }

        if (jsonStartIndex == 0)
        {
          // This shouldn't happen, the line is not well formatted
          // Skip this line
          continue;
        }

        var work = JsonSerializer.Deserialize<Work>(
          line.AsSpan().Slice(jsonStartIndex, line.Length - jsonStartIndex)
        );

        if (work != null)
          writer.WriteLine(work.Title);
      }
    }
  }
}

class Work
{
  [JsonPropertyName("title")]
  public string Title { get; set; }

  public Work()
  {
    Title = "";
  }
}