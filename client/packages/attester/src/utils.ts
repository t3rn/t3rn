import { logger } from './logging'


export function checkKeys(keys: any) {
    // Simple check to see if the keys are present
    const requiredFields = ['btc', 'ethereum', 'substrate']

    try {
        for (const field of requiredFields) {
            if (!keys[field]) {
                logger.error(`Field "${field}" is missing in the JSON.`)
                process.exit(1)
            }
        }

        logger.info(
            {
                substratePublicKey: keys.substrate.publicKey,
                substrateAccountId: keys.substrate.accountId,
                ethereumPublicKey: keys.ethereum.publicKey,
                btcPublicKey: keys.btc.publicKey,
            },
            'Keys are valid'
        )
    } catch (error) {
        logger.error('Invalid Keys JSON', { error })
        process.exit(1)
    }
}
