namespace ParseKata;
using System.Text;

public class CustomParser
{
  private CommandLine.ParserResult<Options> _options { get; set; }
  public CustomParser(CommandLine.ParserResult<Options> options)
  {
    this._options = options;
  }

  public void Parse()
  {
    using (var outputFile = new FileStream(this._options.Value.OutputFilePath, FileMode.Create))
    using (var fs = File.Open(this._options.Value.InputFilePath, FileMode.Open, FileAccess.Read))
    using (var bs = new BufferedStream(fs))
    {
      var buffer = new byte[MAX_BUFFER];
      var bytesRead = 0;

      while ((bytesRead = bs.Read(buffer, 0, MAX_BUFFER)) != 0)
      {
        var titles = this.ExtractTitles(buffer);
        outputFile.Write(titles);
      }
    }
  }

  private static byte[] TitleBytesMarker = Encoding.ASCII.GetBytes("\"title\": \"");
  private static byte QuoteByte = Encoding.ASCII.GetBytes("\"")[0];
  private static byte EscapeByte = Encoding.ASCII.GetBytes("\\")[0];
  private static byte NewLineByte = Encoding.ASCII.GetBytes("\n")[0];
  private const int MAX_BUFFER = 1048576;

  public Span<byte> ExtractTitles(byte[] chunk)
  {
    var chunkSpan = chunk.AsSpan();
    var chunkLength = chunkSpan.Length;

    // Maintain a cursor, which is the index of where we got up to so far
    int chunkCursor = 0;

    // Write any title data we find into this result buffer
    var titles = new byte[MAX_BUFFER];

    // Maintain a cursor, which is the index of how much we wrote to the titles buffer
    int titlesCursor = 0;

    int titleBytesLength = TitleBytesMarker.Length;

    while (chunkCursor < chunkLength)
    {
      // Create a new span to search in, starting from the chunk cursor
      var searchSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);

      // Search for the next set of title bytes
      var nextTitleFieldIndex = searchSpan.IndexOf(TitleBytesMarker);

      // If we can't find a title, break 
      if (nextTitleFieldIndex == -1) break;

      // We found a title, update the title cursor
      chunkCursor = chunkCursor + nextTitleFieldIndex + titleBytesLength;

      // Create a span which starts at the start of the title we found
      var titleSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);

      var foundTitleEndIndex = false;
      var maybeTitleEndIndex = titleSpan.IndexOf(QuoteByte);

      // Try to find the quote byte at the end of the title
      // skipping over escaped quotation marks
      while (!foundTitleEndIndex)
      {
        // If we can't find another quotation mark character, break
        if (maybeTitleEndIndex == -1) break;

        // If the character before the quotation mark is a escape byte (backslash)
        if (maybeTitleEndIndex > 0 && titleSpan[maybeTitleEndIndex - 1] == EscapeByte)
        {
          // Advance the cursor past the escaped quotation we already found
          var adjustedIndex = maybeTitleEndIndex + 1;

          // Search again
          var nextQuoteSearch = titleSpan.Slice(adjustedIndex, titleSpan.Length - adjustedIndex);
          var nextQuoteSearchIndex = nextQuoteSearch.IndexOf(QuoteByte);

          // Adjust the original search index 
          maybeTitleEndIndex = nextQuoteSearchIndex > -1 ? adjustedIndex + nextQuoteSearchIndex : -1;
        }
        else
          foundTitleEndIndex = true;
      }

      // TODO: Handle the case where the end if the chunk is in the middle of a title
      if (maybeTitleEndIndex == -1) break;

      // Create a span which has only the title in it
      var title = titleSpan.Slice(0, maybeTitleEndIndex);

      // Loop over the title and set it's characters into the result buffer
      // incrementing the titles cursor
      for (int i = 0; i < title.Length; i++)
      {
        titles[titlesCursor] = title[i];
        titlesCursor++;
      }

      // Append a new line after the title we just wrote
      // incrementing the titles cursor
      titles[titlesCursor] = NewLineByte;
      titlesCursor++;

      // Update the chunk cursor past the end of the title we processed, the extra
      // 50 chars here are because we know for sure (based on analysing the input file)
      // that the next title is more than 50 chars away
      chunkCursor = chunkCursor + maybeTitleEndIndex + 50;
    }

    // Return the data from the start, up to where we filled it to
    return titles.AsSpan().Slice(0, titlesCursor);
  }
}
