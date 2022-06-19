namespace ParseKata.Tests;
using System.Text;
using ParseKata;

public class LogicTests
{
  [Fact]
  public void ExtractTitles_Single_Title()
  {
    var titleBytes = Encoding.UTF8.GetBytes("/type/work	/works/OL10000049W	3	2010-04-28T06:54:19.472104	{\"title\": \"Le crime organise/quatre films exemplaires\", \"created\": {\"type\": \"/type/datetime\", \"value\": \"2009-12-11T01:57:19.964652\"}, \"covers\": [3140091], \"last_modified\": {\"type\": \"/type/datetime\", \"value\": \"2010-04-28T06:54:19.472104\"}, \"latest_revision\": 3, \"key\": \"/works/OL10000049W\", \"authors\": [{\"type\": \"/type/author_role\", \"author\": {\"key\": \"/a/OL3964979A\"}}], \"type\": {\"key\": \"/type/work\"}, \"id\": 45170289, \"revision\": 3}");
    var result = Logic.ExtractTitles(titleBytes);
    Assert.Equal("Le crime organise/quatre films exemplaires\n", Encoding.UTF8.GetString(result));
  }

  [Fact]
  public void ExtractTitles_Multiple_Titles()
  {
    var titleBytes = Encoding.UTF8.GetBytes("/type/work	/works/OL10000445W	3	2010-04-28T06:54:19.472104	{\"title\": \"Noir comme un ange\", \"created\": {\"type\": \"/type/datetime\", \"value\": \"2009-12-11T01:57:19.964652\"}, \"covers\": [3140866], \"last_modified\": {\"type\": \"/type/datetime\", \"value\": \"2010-04-28T06:54:19.472104\"}, \"latest_revision\": 3, \"key\": \"/works/OL10000445W\", \"authors\": [{\"type\": \"/type/author_role\", \"author\": {\"key\": \"/authors/OL3965493A\"}}], \"type\": {\"key\": \"/type/work\"}, \"revision\": 3}\n/type/work	/works/OL10000766W	3	2010-04-28T06:54:19.472104	{\"title\": \"Le chat sacr\u00e9 de Birmanie\", \"created\": {\"type\": \"/type/datetime\", \"value\": \"2009-12-11T01:57:19.964652\"}, \"covers\": [3141455], \"last_modified\": {\"type\": \"/type/datetime\", \"value\": \"2010-04-28T06:54:19.472104\"}, \"latest_revision\": 3, \"key\": \"/works/OL10000766W\", \"authors\": [{\"type\": \"/type/author_role\", \"author\": {\"key\": \"/authors/OL3965845A\"}}], \"type\": {\"key\": \"/type/work\"}, \"revision\": 3}");
    var result = Logic.ExtractTitles(titleBytes);
    Assert.Equal("Noir comme un ange\nLe chat sacr\u00e9 de Birmanie\n", Encoding.UTF8.GetString(result));
  }
}


