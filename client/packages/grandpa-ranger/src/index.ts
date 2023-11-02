import { GrandpaRanger } from './ranger'
import { logger } from './logging'
;(async () => {
  let config: any
  try {
    config = require(`../config/${process.env.PROFILE}.ts`).default
    logger.info(`Using ${process.env.PROFILE}.ts profile`)
  } catch {
    config = require('../config/local.ts').default
    logger.info('Using local profile')
  }

  const grandpaRanger = new GrandpaRanger(config)
  await grandpaRanger.start()
})()
