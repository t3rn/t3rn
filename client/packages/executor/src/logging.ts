import pino from 'pino'
import fs from 'fs'

// Determine if pretty printing is enabled based on the PROFILE environment variable
const isPrettyPrintEnabled =
    process.env.PROFILE === 'local' || process.env.LOG_PRETTY === 'true'

const { stderr } = process
// Create a writable stream that discards the output
// const NullWritable = fs.createWriteStream('/dev/null')

// Redirect stdout to the NullWritable stream
// stdout.write = NullWritable.write.bind(NullWritable)
// stderr.write = NullWritable.write.bind(NullWritable)

const loggerConfig = {
    level: process.env.LOG_LEVEL || 'info',
    formatters: {
        level: (label) => {
            return { level: label }
        },
    },
    base: undefined,
    stream: process.stdout,
    transport: isPrettyPrintEnabled
        ? {
              target: 'pino-pretty',
          }
        : undefined,
}

export const logger = pino(loggerConfig)
