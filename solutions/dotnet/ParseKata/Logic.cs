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
    int cursor = 0;

    for (int i = 0; i < chunk.Length; i++)
    {
      // Try to match title field bytes
      var foundTitleField = true;
      for (int j = 0; j < TitleBytes.Length; j++)
      {
        if (i < chunk.Length && chunk[i] != TitleBytes[j])
        {
          foundTitleField = false;
          break;
        }

        if (i < chunk.Length)
        {
          i++;
        }
        else
        {
          foundTitleField = false;
          break;
        }
      }

      if (foundTitleField)
      {
        while (i < chunk.Length && chunk[i] != QuoteByte)
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
    }

    return titles.AsSpan().Slice(0, cursor);
  }
}



