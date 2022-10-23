export const parseLine = (chunk: string): string => {
  const jsonStartIndex = chunk.indexOf("{");

  if (jsonStartIndex === -1) {
    // This shouldn't happen, the line is not well formatted
    // Skip this line
    return ""
  }

  const work = JSON.parse(chunk.substring(jsonStartIndex))
  return work.title
}