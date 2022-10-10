#!/usr/bin/env node
import yargs from 'yargs'
import { hideBin } from 'yargs/helpers'
import * as fs from 'fs'
import events from 'events'
import * as readline from 'node:readline/promises'
import { parseLine } from './helpers'
import { resolve } from 'path'

const execute = async () => {
  const { inputFilePath, outputFilePath } = yargs(hideBin(process.argv))
    .option('inputFilePath', {
      alias: 'i',
      describe: 'Input file path'
    })
    .option('outputFilePath', {
      alias: 'o',
      describe: 'Output file path'
    })
    .demandOption(['inputFilePath', 'outputFilePath'])
    .help()
    .argv as { inputFilePath: string, outputFilePath: string }

  const rl = readline.createInterface({
    input: fs.createReadStream(resolve(inputFilePath)),
    crlfDelay: Infinity
  })

  const writeStream = fs.createWriteStream(resolve(outputFilePath))
  writeStream.on('error', (error) => {
    console.error(error)
    throw error
  })

  rl.on('line', (line) => {
    const title = parseLine(line)
    writeStream.write(title + "\n")
  })

  await events.once(rl, 'close')
  writeStream.close()
}

execute().catch(console.error)