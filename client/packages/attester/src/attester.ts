import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import * as ethUtil from 'ethereumjs-util'
import { Mutex } from 'async-mutex'

import { hexToU8a } from '@polkadot/util'

export class Attester {
    circuit: Connection
    config: any
    prometheus: Prometheus
    keys: any
    mutex: any

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus()
        this.keys = keys
        this.mutex = new Mutex()
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
            const event = {
                method: 'NewAttestationMessageHash',
                section: 'attesters',
                index: '0x6505',
                data: [
                    'sepl',
                    '0xcd0116ff1215cb38acd4a5bcff75046b920eef860f4c1b5d176c3dd454862cf3',
                    'EVM',
                ],
            }
            // const event = {
            //     "method": "NewAttestationMessageHash",
            //     "section": "attesters",
            //     "index": "0x6505",
            //     "data": [
            //       "sepl",
            //       "0xd53de5f3bf5d9615c04ef9931460269bff6ddc7285411ea3ca3937a32ccdfeaf",
            //       "EVM"
            //     ]
            //   }

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

                    if (event.section == 'attesters') {
                        logger.debug(
                            { record: record.toHuman() },
                            'Attestation Event'
                        )
                        this.prometheus.eventsAttestationsTotal.inc({
                            method: event.method,
                        })

                        logger.debug(
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
                                logger.debug(
                                    `Received CurrentPendingAttestationBatches event`
                                )

                                const target = event.data[0]
                                // Attest all pending attestations
                                // TODO: remove slice
                                // logger.debug([event.data[1].slice(0, 1)])
                                event.data[1]
                                    .slice(0, 1)
                                    .forEach(async (batch) => {
                                        // Generate the signature for the message hash
                                        await this.submitAttestationEVM(
                                            batch[1],
                                            target,
                                            'EVM'
                                        )
                                    })
                                break
                            }
                            case 'NewTargetProposed': {
                                logger.info(`Received NewTargetProposed event`)
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
        executionVendor: string
    ) {
        const privateKey = Buffer.from(hexToU8a(this.keys.ethereum.privateKey))

        // const messageUint8Array = new Uint8Array(Buffer.from(messageHash.slice(2)))
        const messageUint8Array = ethUtil.hashPersonalMessage(
            ethUtil.toBuffer(messageHash)
        )
        // logger.debug(['MessageHash', messageHash])
        const sigObj = ethUtil.ecsign(messageUint8Array, privateKey)

        const signature = ethUtil.toRpcSig(sigObj.v, sigObj.r, sigObj.s)

        const tx = this.circuit.client.tx.attesters.submitAttestation(
            messageHash,
            signature,
            targetId
        )

        // Before attestation we need to check if we are in current committee
        if (!(await this.isInCommittee())) {
            return
        }

        let result
        try {
            await this.mutex.runExclusive(async () => {
                logger.debug(
                    {
                        executionVendor: executionVendor.toString(),
                        targetId: targetId,
                        messageHash: messageHash,
                        signature: signature.toString(),
                    },
                    'Submitting attestation'
                )
                result = await this.circuit.sdk?.circuit.tx.signAndSendSafe(tx)
            })
        } catch (error) {
            const errorName = error.stack.match(/^Error: (.*):/)[1] // Parse stack trace to get exact error which is not present in error object
            logger.error(
                {
                    error: errorName,
                    messageHash: messageHash,
                    targetId: targetId,
                    executionVendor: executionVendor,
                },
                'Error submitting attestation'
            )
            this.prometheus.submitAttestationError.inc({ error: errorName })
            return
        }

        logger.debug(result)
        this.prometheus.submittedAttestation.inc({
            messageHash: messageHash,
            targetId: targetId,
            executionVendor: executionVendor,
        })

        logger.info(
            {
                executionVendor: executionVendor,
                targetId: targetId,
                messageHash: messageHash,
                // hash: result.hash.toHex(),
            },
            'Attestation submitted'
        )
    }

    private async isInCommittee() {
        let committee
        try {
            committee =
                await this.circuit.client.query.attesters.currentCommittee()
        } catch (error) {
            logger.error(error.stack, 'Error getting committee')
            this.prometheus.currentCommitteeMember.set(0)
            return
        }

        const isInCommittee =
            committee.find(
                (item) => item.accountId === this.keys.substrate.addressId
            ) !== undefined
        logger.info({ committee: isInCommittee }, 'Current committee member')
        this.prometheus.currentCommitteeMember.set(isInCommittee ? 1 : 0)
        return isInCommittee
    }
}
