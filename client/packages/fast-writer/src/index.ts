import { FastWriter } from './services/fast-writer/fast-writer.class'
import { logger } from './utils/logger'
import { Config } from './config/config'
import { cryptoWaitReady } from '@t3rn/sdk'

process.on('unhandledRejection', (reason, promise) => {
  logger.error({promise, reason}, 'Unhandled Rejection. Shutting down...')
  process.exit(1)
})

async function main() {
  const config: Config = (await import(`./config/config`)).default

  const cryptoIsReady = await cryptoWaitReady()
  if (!cryptoIsReady) {
    throw new Error('Crypto WASM lib is not ready')
  }

  logger.info('Starting Fast Writer')

  const fastWriter = new FastWriter(config)
  await fastWriter.start()
}

main()
