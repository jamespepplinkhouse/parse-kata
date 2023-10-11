import java.io._
import org.json._
import java.time._

object ParseKata {
  def main(args: Array[String]): Unit = {
    if (args.length != 2) {
    System.err.println("Usage: scala MainApp <inputFilePath> <outputFilePath>")
    System.exit(1)
}

    val inputFilePath = args(0)
    val outputFilePath = args(1)
    val startTime = Instant.now
    
    try {
      val writer = new PrintWriter(new File(outputFilePath))

      for (line <- scala.io.Source.fromFile(inputFilePath).getLines) {
        val jsonStart = line.indexOf('{')
        if (jsonStart != -1) {
          val jsonString = line.substring(jsonStart)
          try {
            val json = new JSONObject(jsonString)
            val title = json.getString("title")
            writer.write(title + "\n")
          } catch {
            case e: JSONException => println("Error parsing JSON: " + e.getMessage)
          }
        } else {
          println("No JSON found in line.")
        }
      }
      writer.close()

      val endTime = Instant.now

      println("Elapsed time: " + Duration.between(startTime, endTime))
    } catch {
      case e: Exception => println(s"Error processing file: ${e.getMessage}")
    }
  }
}