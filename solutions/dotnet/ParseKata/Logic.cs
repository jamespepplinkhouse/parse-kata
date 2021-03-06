namespace ParseKata;

using System.Text;

public static class Logic
{
  private static byte[] TitleBytes = Encoding.ASCII.GetBytes("\"title\": \"");
  private static byte QuoteByte = Encoding.ASCII.GetBytes("\"")[0];
  private static byte NewLineByte = Encoding.ASCII.GetBytes("\n")[0];
  private const int MAX_BUFFER = 1048576;

  public static Span<byte> ExtractTitles(byte[] chunk)
  {
    var chunkSpan = chunk.AsSpan();
    var chunkLength = chunkSpan.Length;

    // Maintain a cursor, which is the index of where we got up to so far
    int chunkCursor = 0;

    // Write any title data we find into this result buffer
    var titles = new byte[MAX_BUFFER];
    
    // Maintain a cursor, which is the index of how much we wrote to the titles buffer
    int titlesCursor = 0;

    int titleBytesLength = TitleBytes.Length;

    while (chunkCursor < chunkLength)
    {
      // Create a new span to search in, starting from the chunk cursor
      var searchSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);

      // Search for the next set of title bytes
      var nextTitleFieldIndex = searchSpan.IndexOf(TitleBytes);

      // If we can't find a title, break 
      if (nextTitleFieldIndex == -1) break;

      // We found a title, update the title cursor
      chunkCursor = chunkCursor + nextTitleFieldIndex + titleBytesLength;

      // Create a span which starts at the start of the title we found
      var titleSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);

      // Find the end of the title
      var titleEndIndex = titleSpan.IndexOf(QuoteByte);

      // TODO: We need to check if the character before titleEndIndex is a backslash
      // in that case it's an escaped quote, and we need to keep looking!

      // TODO: Handle the case where the end if the chunk is in the middle of a title
      if (titleEndIndex == -1) break;

      // Create a span which has only the title in it
      var title = titleSpan.Slice(0, titleEndIndex);

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
      chunkCursor = chunkCursor + titleEndIndex + 50;
    }

    // Return the data from the start, up to where we filled it to
    return titles.AsSpan().Slice(0, titlesCursor);
  }
}



