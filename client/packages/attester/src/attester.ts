import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import ethUtil from 'ethereumjs-util'
import { hexToU8a } from '@polkadot/util'

export class Attester {
    circuit: Connection
    config: any
    prometheus: Prometheus
    keys: any

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus()
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
            this.keys.substrate.privateKey
        )
        this.circuit.connect()
    }

    async listenEvents() {
        logger.info('Listening to events')
        // Subscribe to the NewAttestationMessageHash event
        this.circuit.sdk?.client.query.system.events(async (events) => {
            logger.info({ events_count: events.length }, `Received events`)
            this.prometheus.eventsTotal.inc(events.length)
            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record
                    logger.debug(event.toHuman())

                    if (event.section == 'attesters') {
                        this.prometheus.eventsAttestationsTotal.inc().set({
                            method: event.method,
                        })
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
