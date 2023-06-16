// eslint-disable-next-line @typescript-eslint/no-var-requires
require('dotenv').config()
import { logger } from './logging'
import { Attester } from './attester'
import { checkKeys } from './utils'
import { Prometheus } from './prometheus'
import { Connection } from './connection'

process.on('uncaughtException', (error) => {
    logger.error(error.stack)
    process.exit(1)
})
;(async () => {
    logger.info(`Starting attester`)

    let config: any
    if (process.env.PROFILE === 'prod') {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require('../config/prod.ts').default
    } else if (process.env.PROFILE === 'roco') {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require('../config/roco.ts').default
    } else {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require('../config/local.ts').default
    }

    let keys: any
    try {
        keys = JSON.parse(
            Buffer.from(process.env.KEYS as string, 'base64').toString('utf-8')
        )
    } catch (error) {
        logger.error('Invalid Keys JSON', { error })
        process.exit(1)
    }

    checkKeys(keys)
    const attester = new Attester(config, keys)
    await attester.start()
})()

export { Attester, Prometheus, Connection, checkKeys }
