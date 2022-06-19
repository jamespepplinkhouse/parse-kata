using System;
using System.Security.Cryptography;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using ParseKata;
using System.Text;

namespace ParseKata.Benchmark
{
  public class LogicBenchmark
  {
    private byte[] TitleBytes;

    public LogicBenchmark()
    {
      this.TitleBytes = File.ReadAllBytes(@"../../../../../../../../../../samples/1mb.txt");
    }

    [Benchmark]
    public Span<byte> ExtractTitles() => Logic.ExtractTitles(this.TitleBytes);

  }

  public class Program
  {
    public static void Main(string[] args)
    {
      var summary = BenchmarkRunner.Run(typeof(Program).Assembly);
    }
  }
}