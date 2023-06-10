import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import * as ethUtil from 'ethereumjs-util'

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

        if (process.env.TEST == 'true') {
            const event = {
                method: 'NewAttestationMessageHash',
                section: 'attesters',
                index: '0x6505',
                data: [
                    '0x7365706C',
                    '0xe8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c',
                    'EVM',
                ],
            }

            const [targetId, messageHash, executionVendor] = event.data
            await this.submitAttestationEVM(
                messageHash,
                targetId,
                executionVendor
            )
        } else {
            this.listenEvents()
        }
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
            logger.debug({ events_count: events.length }, `Received events`)
            this.prometheus.eventsTotal.inc(events.length)

            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record
                    logger.debug({ event: event }, 'Event data')

                    if (event.section == 'attesters') {
                        this.prometheus.eventsAttestationsTotal.inc({
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
                                        messageHash: messageHash,
                                        executionVendor:
                                            executionVendor.toString(),
                                    },
                                    `Received the attestation message hash request to sign`
                                )
                                // Submit the attestation for the given target ID for the given message hash for each attester's key in the keys.json file
                                if (executionVendor.toString() == 'Substrate') {
                                    logger.warn('Substrate not implemented')
                                } else if (
                                    executionVendor.toString() == 'Ed25519'
                                ) {
                                    logger.warn('Ed25519 not implemented')
                                } else if (
                                    executionVendor.toString() == 'EVM'
                                ) {
                                    // Generate the signature for the message hash
                                    try {
                                        await this.submitAttestationEVM(
                                            messageHash.toHex(),
                                            targetId,
                                            executionVendor
                                        )
                                    } catch (error) {
                                        logger.error(
                                            error,
                                            'Error submitting attestation'
                                        )
                                        return
                                    }
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

    private async submitAttestationEVM(
        messageHash: any,
        targetId: any,
        executionVendor: any
    ) {
        const privateKey = Buffer.from(hexToU8a(this.keys.ethereum.privateKey))

        const sigObj = ethUtil.ecsign(
            Buffer.from(hexToU8a(messageHash)),
            privateKey
        )

        const signature = ethUtil.toRpcSig(sigObj.v, sigObj.r, sigObj.s)

        const tx = this.circuit.client.tx.attesters.submitAttestation(
            messageHash,
            signature,
            targetId
        )
        logger.info(
            {
                executionVendor: executionVendor.toString(),
                targetId: targetId,
                messageHash: messageHash,
                signature: signature.toString(),
            },
            'Submitting attestation'
        )

        let result
        try {
            result = await this.circuit.sdk?.circuit.tx.signAndSendSafe(tx)
        } catch (error) {
            logger.error(error, 'Error submitting attestation')
            return
        }

        logger.error(result)

        logger.info(
            {
                executionVendor: executionVendor.toString(),
                targetId: targetId.toString(),
                messageHash: messageHash,
                hash: result.hash.toHex(),
            },
            'Attestation submitted'
        )
    }
}
