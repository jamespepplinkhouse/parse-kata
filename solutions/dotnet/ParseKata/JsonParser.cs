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

  public void Parse()
  {
    using (StreamWriter writer = new StreamWriter(this._options.OutputFilePath))
    {
      var titles = File.ReadLines(this._options.InputFilePath)
              .AsParallel()
              .AsOrdered()
              .Select((line) =>
              {
                var jsonStartIndex = line.IndexOf('{');
                if (jsonStartIndex == -1)
                  return "";

                return this.ExtractTitle(line.AsSpan().Slice(jsonStartIndex, line.Length - jsonStartIndex));
              });

      foreach (var title in titles)
        writer.WriteLine(title);
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