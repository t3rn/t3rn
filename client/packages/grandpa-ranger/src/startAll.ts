import { GrandpaRanger } from './ranger'
import { logger } from './logging'
;(async () => {
  let configRococo: any
  try {
    logger.info(`👨‍🦳trying to load GRANDPA ranger for Rococo...`)
    // Set process.env.PROFILE to 'polkadot' to use this config
    process.env.PROFILE = 'rococo'
    configRococo = require(`../config/${process.env.PROFILE}.ts`).default
    logger.info(`Using ${process.env.PROFILE}.ts profile`)
  } catch {
    logger.fatal(`🪦 GRANDPA ranger for Rococo failed to load!`)
    // Set process.env.PROFILE to 'local' to use this config
    // Sleep for 10 minutes to allow for manual intervention
    await new Promise(resolve => setTimeout(resolve, 600000))
    process.exit(1)
  }

  const grandpaRangerRococo = new GrandpaRanger(configRococo)
  await grandpaRangerRococo.start()

  let configKusama: any
  try {
    logger.info(`👨‍🦳trying to load GRANDPA ranger for Kusama...`)
    // Set process.env.PROFILE to 'polkadot' to use this config
    process.env.PROFILE = 'kusama'
    configKusama = require(`../config/${process.env.PROFILE}.ts`).default
    logger.info(`Using ${process.env.PROFILE}.ts profile`)
  } catch {
    logger.fatal(`🪦 GRANDPA ranger for Kusama failed to load!`)
    // Set process.env.PROFILE to 'local' to use this config
    // Sleep for 10 minutes to allow for manual intervention
    await new Promise(resolve => setTimeout(resolve, 600000))
    process.exit(1)
  }

  const grandpaRangerKusama = new GrandpaRanger(configKusama)
  await grandpaRangerKusama.start()

  let configPolkadot: any
  try {
    logger.info(`👨‍🦳trying to load GRANDPA ranger for Polkadot...`)
    // Set process.env.PROFILE to 'polkadot' to use this config
    process.env.PROFILE = 'polkadot'
    configPolkadot = require(`../config/${process.env.PROFILE}.ts`).default
    logger.info(`Using ${process.env.PROFILE}.ts profile`)
  } catch {
    logger.fatal(`🪦 GRANDPA ranger for Polkadot failed to load!`)
    // Set process.env.PROFILE to 'local' to use this config
    // Sleep for 10 minutes to allow for manual intervention
    await new Promise(resolve => setTimeout(resolve, 600000))
    process.exit(1)
  }

  const grandpaRangerPolkadot = new GrandpaRanger(configPolkadot)
  await grandpaRangerPolkadot.start()
})()
