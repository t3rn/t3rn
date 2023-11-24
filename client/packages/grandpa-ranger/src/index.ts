import { GrandpaRanger } from './ranger'
import { logger } from './logging'
;(async () => {
  let config: any
  try {
    if (process.env.CIRCUIT == 't2rn') {
      config = require(`../config/${process.env.PROFILE}.t2rn.ts`).default
      logger.info(`Using ${process.env.PROFILE}.t2rn.ts profile`)
    } else {
      config = require(`../config/${process.env.PROFILE}.ts`).default
      logger.info(`Using ${process.env.PROFILE}.ts profile`)
    }
  } catch {
    config = require('../config/local.ts').default
    logger.info('Using local profile')
  }

  const grandpaRanger = new GrandpaRanger(config)
  await grandpaRanger.start()
})()
