export const parseLine = (chunk: string): string => {
  var jsonStartIndex = 0

  for (let i = 0; i < 4; i++) {
    var newIndex = chunk.indexOf('\t', jsonStartIndex) + 1
    if (newIndex == -1) {
      jsonStartIndex = 0
      break
    }
    jsonStartIndex = newIndex
  }

  if (jsonStartIndex == 0) {
    // This shouldn't happen, the line is not well formatted
    // Skip this line
    return ""
  }

  const work = JSON.parse(chunk.substring(jsonStartIndex))
  return work.title
}