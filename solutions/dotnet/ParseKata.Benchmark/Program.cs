using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;

namespace ParseKata.Benchmark
{
  public class LogicBenchmark
  {
    private byte[] TitleBytes;

    public LogicBenchmark()
    {
      this.TitleBytes = File.ReadAllBytes(@"../../../../../../../../../../samples/1mb-input.txt");
    }

    [Benchmark]
    public Span<byte> ExtractTitles()
    {
      var parser = new CustomParser(null);
      return parser.ExtractTitles(this.TitleBytes);
    }
  }

  public class Program
  {
    public static void Main(string[] args)
    {
      var summary = BenchmarkRunner.Run(typeof(Program).Assembly);
    }
  }
}