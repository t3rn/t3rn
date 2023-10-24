import pino from 'pino'

// Determine if pretty printing is enabled based on the PROFILE environment variable
const isPrettyPrintEnabled =
  process.env.PROFILE === 'local' || process.env.LOG_PRETTY === 'true'

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
