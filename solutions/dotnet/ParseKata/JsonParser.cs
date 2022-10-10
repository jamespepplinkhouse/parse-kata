namespace ParseKata;
using System.Text.Json;
using System.Text.Json.Serialization;

public class JsonParser
{
  private Options _options { get; set; }
  public JsonParser(Options options)
  {
    this._options = options;
  }

  public async void Parse()
  {
    using (StreamWriter writer = new StreamWriter(this._options.OutputFilePath))
    {
      await foreach (var line in File.ReadLinesAsync(this._options.InputFilePath))
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

        var title = this.ExtractTitle(line.AsSpan().Slice(jsonStartIndex, line.Length - jsonStartIndex));
        await writer.WriteLineAsync(title);
      }
    }
  }

  public string ExtractTitle(System.ReadOnlySpan<char> rawJson)
  {
    var work = JsonSerializer.Deserialize<Work>(
      rawJson
    );

    return (work != null) ? work.Title : string.Empty;
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