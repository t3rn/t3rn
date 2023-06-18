import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import * as ethUtil from 'ethereumjs-util'
import queue from 'async/queue'

import { hexToU8a } from '@polkadot/util'

/**
 * @group Attester
 */
export class Attester {
    circuit: Connection
    config: any
    prometheus: Prometheus
    keys: any
    isInCurrentCommittee = false
    q: any
    boundProcessAttestation: any

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus()
        this.keys = keys
        this.boundProcessAttestation = this.processAttestation.bind(this)

        this.q = queue(this.boundProcessAttestation, 1)
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
            logger.debug({ events_count: events.length }, `Received events`)
            this.prometheus.eventsTotal.inc(events.length)

            const comittee = await this.getCommittee()
            this.checkIsInCommittee(comittee, this.keys.substrate.accountId)

            const attesterEvents = events.filter(
                async (event) => event.section == 'attesters'
            )

            this.prometheus.attestationsEvents.inc(attesterEvents.length)

            await this.processAttestationEvents(attesterEvents)
        })
    }

    private async processAttestationEvents(events: any) {
        // Loop through the Vec<EventRecord>
        await Promise.all(
            events.map(async (record) => {
                // Extract the phase, event and the event types
                const { event } = record

                switch (event.method) {
                    case 'NewAttestationMessageHash': {
                        const [targetId, messageHash, executionVendor] =
                            event.data

                        if (executionVendor.toString() == 'Substrate') {
                            logger.warn('Substrate not implemented')
                        } else if (executionVendor.toString() == 'Ed25519') {
                            logger.warn('Ed25519 not implemented')
                        } else if (executionVendor.toString() == 'EVM') {
                            // Add attestion to queue
                            this.q.push({
                                messageHash: messageHash,
                                targetId: targetId,
                                executionVendor: 'EVM',
                            })
                            this.prometheus.attestationsInQueue.set(
                                this.q.length()
                            )
                        }
                        break
                    }
                    case 'CurrentPendingAttestationBatches': {
                        const targetId = event.data[0].toString()
                        const messageHashes = event.data[1]

                        logger.info(
                            {
                                targetId: targetId,
                                messageHashes: messageHashes.length,
                            },
                            `Received CurrentPendingAttestationBatches event`
                        )
                        this.prometheus.attestionsPending.set(
                            {
                                targetId: targetId,
                            },
                            messageHashes.length
                        )

                        // Purge queue
                        this.queuePurge()

                        messageHashes.forEach(async (messageHash) => {
                            if (
                                !this.config.targetsAllowed.includes(targetId)
                            ) {
                                logger.warn(
                                    {
                                        targetId: targetId,
                                    },
                                    'Target not allowed'
                                )
                                return
                            }

                            // Add all pending attestations to queue
                            this.q.push({
                                messageHash: messageHash[1],
                                targetId: targetId,
                                executionVendor: 'EVM',
                            })
                        })
                        this.prometheus.attestationsInQueue.set(this.q.length())
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
            })
        )
    }

    private queuePurge() {
        this.q.remove(function testFn(item: { data: any }): boolean {
            return true
        })
    }

    private async processAttestation(data: any) {
        await this.submitAttestationEVM(
            data.messageHash,
            data.targetId,
            data.executionVendor
        )
    }

    private async submitAttestationEVM(
        messageHash: string,
        targetId: string,
        executionVendor: string
    ) {
        if (!this.isInCurrentCommittee) {
            logger.warn('Not in current committee, not submitting attestation')
            return
        }

        const { signature, tx } = this.generateAttestationTx(
            messageHash,
            targetId
        )

        let result
        try {
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
            this.prometheus.attestationSubmitError.inc({ error: errorName })
            return
        }
        if (!result) {
            return
        }

        logger.debug(result)
        this.prometheus.attestationSubmitted.inc({
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

    private generateAttestationTx(messageHash: string, targetId: string) {
        const privateKey = Buffer.from(hexToU8a(this.keys.ethereum.privateKey))

        const messageUint8Array = ethUtil.hashPersonalMessage(
            ethUtil.toBuffer(messageHash)
        )
        const sigObj = ethUtil.ecsign(messageUint8Array, privateKey)

        const signature = ethUtil.toRpcSig(sigObj.v, sigObj.r, sigObj.s)

        const tx = this.circuit.client.tx.attesters.submitAttestation(
            messageHash,
            signature,
            targetId
        )
        return { signature, tx }
    }

    private async getCommittee(): Promise<string[]> {
        let committee
        try {
            committee =
                await this.circuit.client.query.attesters.currentCommittee()
        } catch (error) {
            logger.error(error.stack, 'Error getting committee')
            this.prometheus.currentCommitteeMember.set(0)
            return []
        }

        return Object.values(committee.toJSON()).map((value) => String(value))
    }

    private async checkIsInCommittee(committee: string[], accountId: string) {
        this.isInCurrentCommittee = committee.includes(accountId)

        logger.debug(
            {
                account: accountId,
                committee: committee,
                isInCurrentCommittee: this.isInCurrentCommittee,
            },
            'Current committee member'
        )
        this.prometheus.currentCommitteeMember.set(
            this.isInCurrentCommittee ? 1 : 0
        )
    }
}
