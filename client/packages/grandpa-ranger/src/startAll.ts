import { GrandpaRanger } from './ranger'
import { logger } from './logging'

const startRanger = async (profile: string) => {
  let config: any
  try {
    logger.info(`👨‍🦳 trying to load GRANDPA ranger for ${profile}...`)
    process.env.PROFILE = profile
    config = require(`../config/${profile}.ts`).default
    logger.info(`Using ${profile}.ts profile`)
  } catch (error) {
    logger.fatal(`🪦 GRANDPA ranger for ${profile} failed to load!`)
    await new Promise(resolve => setTimeout(resolve, 600000))
    process.exit(1)
  }

  const ranger = new GrandpaRanger(config)
  await ranger.start()
}

;(async () => {
  try {
    await Promise.all([
      startRanger('rococo'),
      startRanger('kusama'),
      startRanger('polkadot'),
    ])
  } catch (error) {
    logger.error(
      { error },
      `An error occurred while starting the rangers for ${process.env.PROFILE}`,
    )
    process.exit(1)
  }
})()
