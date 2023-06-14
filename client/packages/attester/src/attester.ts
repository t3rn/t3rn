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
            logger.debug('Testing mode')
            // const event = {
            //     method: 'NewAttestationMessageHash',
            //     section: 'attesters',
            //     index: '0x6505',
            //     data: [
            //         '0x7365706C',
            //         '0xe8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c',
            //         'EVM',
            //     ],
            // }
            const event = {
                "method": "NewAttestationMessageHash",
                "section": "attesters",
                "index": "0x6505",
                "data": [
                  "sepl",
                  "0xd53de5f3bf5d9615c04ef9931460269bff6ddc7285411ea3ca3937a32ccdfeaf",
                  "EVM"
                ]
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

            // Before acting on attestation events we need to check if we are in current committee
            if (!await this.isInCommittee()) {
                logger.debug('Not in committee')
                return
            }
            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record

                    if (event.section == 'attesters') {
                        logger.debug({ record: record.toHuman() }, 'Attestation Event')
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
                                    `Received NewAttestationMessageHash event to sign`
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
                                        await this.submitAttestationEVM(
                                            messageHash,
                                            targetId,
                                            executionVendor
                                        )
                                }

                                break
                            }
                            case 'CurrentPendingAttestationBatches': {
                                logger.info(
                                    `Received CurrentPendingAttestationBatches event`
                                )

                                logger.info(event.data)
                                const target = event.data[0]
                                // Attest all pending attestations
                                // TODO: remove slice
                                event.data[1].slice(0, 1).forEach(async (batch) => {
                                    // Generate the signature for the message hash
                                        await this.submitAttestationEVM(
                                            batch[1],
                                            target,
                                            "EVM"
                                    )
                                })
                                break
                            }
                            case 'NewTargetProposed': {
                                logger.info(
                                    `Received NewTargetProposed event`
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
        messageHash: string,
        targetId: string,
        executionVendor: string,
    ) {
        const privateKey = Buffer.from(hexToU8a(this.keys.ethereum.privateKey))

        // const messageUint8Array = new Uint8Array(Buffer.from(messageHash.slice(2)))
        const messageUint8Array = ethUtil.hashPersonalMessage(ethUtil.toBuffer(messageHash))
        // logger.debug(['MessageHash', messageHash])
        const sigObj = ethUtil.ecsign(
            messageUint8Array,
            privateKey,
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
            logger.error(error.stack, 'Error submitting attestation')
            // process.exit(17)
            return
        }

        logger.debug(result)

        logger.info(
            {
                executionVendor: executionVendor.toString(),
                targetId: targetId.toString(),
                messageHash: messageHash,
                // hash: result.hash.toHex(),
            },
            'Attestation submitted'
        )
    }

    private async isInCommittee() {
        let comittee
        try {
            comittee = await this.circuit.client.query.attesters.currentCommittee()
        } catch (error) {
            logger.error(error.stack, 'Error getting committee')
            return
        }

        return comittee.includes(this.keys.substrate.addressId)
    }

}