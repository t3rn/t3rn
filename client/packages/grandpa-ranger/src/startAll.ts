import { GrandpaRanger } from './ranger'
import { logger } from './logging'

const loadConfig = async (profile: string) => {
  let config: any
  try {
    // Sleep for 3sec to de-sync the rangers interval
    await new Promise((resolve, _reject) => setTimeout(resolve, 3000))
    if (process.env.CIRCUIT === 't2rn') {
      config = require(`../config/${profile}.t2rn.ts`).default
      logger.info(`Using ${profile}.t2rn.ts profile`)
    } else {
      config = require(`../config/${profile}.ts`).default
      logger.info(`Using ${profile}.ts profile`)
    }
  } catch {
    config = require('../config/local.ts').default
    logger.info('Using local profile')
  }
  return config
}

const startRanger = async (profile: string) => {
  const config = await loadConfig(profile)
  const ranger = new GrandpaRanger(config)
  await ranger.start()
}

;(async () => {
  const profiles = ['rococo', 'kusama', 'polkadot']

  try {
    await Promise.all(profiles.map(profile => startRanger(profile)))
  } catch (error) {
    logger.fatal('An error occurred while starting the rangers:', error)
    process.exit(1)
  }
})()
