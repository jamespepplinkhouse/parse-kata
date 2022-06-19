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
    var titleStartIndexes = titleSearch.SearchAll(chunk);
    int cursor = 0;

    foreach (var titleIndex in titleStartIndexes)
    {
      int i = titleIndex + TitleBytes.Length;
      while (chunk[i] != QuoteByte)
      {
        titles[cursor] = chunk[i];
        cursor++;

        if (i < chunk.Length)
        {
          i++;
        }
        else
        {
          break;
        }
      }
      titles[cursor] = NewLineByte;
      cursor++;
    }

    return titles.AsSpan().Slice(0, cursor);
  }
}



