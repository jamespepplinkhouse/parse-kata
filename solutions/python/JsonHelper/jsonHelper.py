from cmath import log
import json
import string


def extractJsonFromString(stringToParse: string) -> string:
    json_start_index = stringToParse.find("{")
    try:
        result = stringToParse[json_start_index:len(stringToParse)]
    except:
        log("I could not find a valid json string.")
    return result
