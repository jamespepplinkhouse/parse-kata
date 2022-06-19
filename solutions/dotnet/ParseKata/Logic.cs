namespace ParseKata;

using System.Text;

public static class Logic
{
  private static byte[] TitleBytes = Encoding.ASCII.GetBytes("title\": \"");
  private static byte QuoteByte = Encoding.ASCII.GetBytes("\"")[0];
  private static byte NewLineByte = Encoding.ASCII.GetBytes("\n")[0];

  public static byte[] ExtractTitles(byte[] chunk)
  {
    var titles = new List<byte>();

    for (int i = 0; i < chunk.Length; i++)
    {
      // If it's a new newline, skip ahead
      if (chunk[i] == NewLineByte)
      {
        if (i + 49 < chunk.Length)
        {
          i = i + 49;
          continue;
        }
        else
        {
          break;
        }
      }

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
          titles.Add(chunk[i]);
          if (i < chunk.Length)
          {
            i++;
          }
          else
          {
            break;
          }
        }
        titles.Add(NewLineByte);
      }
    }

    return titles.ToArray();
  }
}



