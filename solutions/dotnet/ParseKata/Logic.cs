namespace ParseKata;

using System.Text;

public static class Logic
{
  private static byte[] TitleBytes = Encoding.ASCII.GetBytes("title\": \"");
  private static byte QuoteByte = Encoding.ASCII.GetBytes("\"")[0];
  private static byte NewLineByte = Encoding.ASCII.GetBytes("\n")[0];
  private const int MAX_BUFFER = 1048576;

  public static Span<byte> ExtractTitles(byte[] chunk)
  {
    var chunkSpan = chunk.AsSpan();
    var chunkLength = chunkSpan.Length;
    int chunkCursor = 0;

    var titles = new byte[MAX_BUFFER];
    int titlesCursor = 0;
    int titleBytesLength = TitleBytes.Length;

    while (chunkCursor < chunkLength)
    {
      var searchSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);
      var nextTitleFieldIndex = searchSpan.IndexOf(TitleBytes);
      if (nextTitleFieldIndex == -1) break;

      chunkCursor = chunkCursor + nextTitleFieldIndex + titleBytesLength;
      var titleSpan = chunkSpan.Slice(chunkCursor, chunkLength - chunkCursor);
      var titleEndIndex = titleSpan.IndexOf(QuoteByte);
      if (titleEndIndex == -1) break;

      var title = titleSpan.Slice(0, titleEndIndex);
      for (int i = 0; i < title.Length; i++)
      {
        titles[titlesCursor] = title[i];
        titlesCursor++;
      }

      titles[titlesCursor] = NewLineByte;
      titlesCursor++;

      chunkCursor = chunkCursor + titleEndIndex + 50;
    }

    return titles.AsSpan().Slice(0, titlesCursor);
  }
}



