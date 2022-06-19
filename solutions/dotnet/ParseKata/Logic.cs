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
    var titles = new byte[MAX_BUFFER];
    var titleSearch = new BoyerMoore(TitleBytes);
    var quoteSearch = new BoyerMoore(new byte[QuoteByte]);
    int chunkIndex = 0;
    int titlesCursor = 0;

    while (chunkIndex < chunk.Length)
    {
      var titleStartIndex = titleSearch.Search(chunk, chunkIndex);
      if (titleStartIndex > 0)
      {
        var endQuoteIndex = quoteSearch.Search(chunk, titleStartIndex + TitleBytes.Length);

        if (endQuoteIndex > 0)
        {
          // copy from titleStartIndex + TitleBytes.Length to endQuoteIndex

          // int i = titleStartIndex + TitleBytes.Length;
          // while (chunk[i] != QuoteByte)
          // {
          //   titles[titlesCursor] = chunk[i];
          //   titlesCursor++;

          //   if (i < chunk.Length)
          //   {
          //     i++;
          //   }
          //   else
          //   {
          //     break;
          //   }
          // }
          // titles[titlesCursor] = NewLineByte;
          // titlesCursor++;

          chunkIndex = endQuoteIndex;
        }
        else
        {
          // take to the end of the chunk
          break;
        }
      }
      else
      {
        // No more titles found
        break;
      }
    }

    return titles.AsSpan().Slice(0, titlesCursor);
  }
}



