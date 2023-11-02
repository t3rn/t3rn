import { FastWriter } from './services/fast-writer/fast-writer.class'
import { logger } from './utils/logger'
import { Config } from './config/config'

async function main() {
  const config: Config = (await import(`./config/config`)).default

  logger.info(
    { logLevel: config.log.level },
    'Starting Fast Writer',
  )

  const fastWriter = new FastWriter(config)
  await fastWriter.start()
}

main()
