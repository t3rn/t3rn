// eslint-disable-next-line @typescript-eslint/no-var-requires
require('dotenv').config()
import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import ethUtil from 'ethereumjs-util'
import { hexToU8a } from '@polkadot/util'

class Attester {
    circuit: Connection
    target: Connection
    config: any
    prometheus: Prometheus
    keys: any

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus(this.config.targetGatewayId)
        this.keys = keys
    }

    async start() {
        await this.connectClients()

        await new Promise<void>((resolve) => {
            const checkClient = () => {
                if (this.circuit?.sdk?.client) {
                    resolve()
                } else {
                    setTimeout(checkClient, 2000)
                }
            }
            checkClient()
        })

        this.listenEvents()
    }

    async connectClients() {
        await cryptoWaitReady()
        this.circuit = new Connection(
            this.config.circuit.rpc1,
            this.config.circuit.rpc2,
            this.prometheus,
            this.config.targetGatewayId,
            this.keys.substrate.privateKey
        )
        this.circuit.connect()
    }

    async listenEvents() {
        logger.info('Listening to events')
        // Subscribe to the NewAttestationMessageHash event
        this.circuit.sdk?.client.query.system.events(async (events) => {
            logger.info({ events_count: events.length }, `Received events`)
            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record
                    logger.debug(event.toHuman())

                    if (event.section == 'attesters') {
                        logger.info(
                            {
                                section: event.section,
                                method: event.method,
                                phase: record.phase,
                            },
                            'Received an event for the attester'
                        )

                        switch (event.method) {
                            case 'NewAttestationMessageHash': {
                                const [targetId, messageHash, executionVendor] =
                                    event.data
                                logger.info(
                                    {
                                        targetId: targetId.toString(),
                                        messageHash: messageHash.toHex(),
                                        executionVendor:
                                            executionVendor.toString(),
                                    },
                                    `Received the attestation message hash request to sign`
                                )
                                // Submit the attestation for the given target ID for the given message hash for each attester's key in the keys.json file
                                if (executionVendor.toString() == 'Substrate') {
                                    logger.warn('Substrate unhandled yet')
                                } else if (
                                    executionVendor.toString() == 'Ed25519'
                                ) {
                                    logger.warn('Ed25519 unhandled yet')
                                } else if (
                                    executionVendor.toString() == 'EVM'
                                ) {
                                    // Generate the signature for the message hash
                                    const privateKey = Buffer.from(
                                        hexToU8a(
                                            this.config.keys.ethereum.privateKey
                                        )
                                    )

                                    const sigObj = ethUtil.ecsign(
                                        Buffer.from(
                                            hexToU8a(messageHash.toHex())
                                        ),
                                        privateKey
                                    )
                                    const signature = ethUtil.toRpcSig(
                                        sigObj.v,
                                        sigObj.r,
                                        sigObj.s
                                    )

                                    const tx =
                                        this.circuit.client.tx.attesters.submitAttestation(
                                            messageHash,
                                            signature,
                                            targetId
                                        )

                                    logger.info(
                                        {
                                            executionVendor:
                                                executionVendor.toString(),
                                            targetId: targetId.toString(),
                                            messageHash: messageHash.toHex(),
                                            hash: tx.hash.toHex(),
                                        },
                                        'Executed'
                                    )
                                }

                                break
                            }
                            case 'NewTargetProposed': {
                                logger.info(
                                    `Received the new target proposed event`
                                )
                                break
                            }
                            default: {
                                break
                            }
                        }
                    }
                })
            )
        })
    }
}

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

function checkKeys(keys: any) {
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
