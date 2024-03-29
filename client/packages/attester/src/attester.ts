import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import queue from 'async/queue'
import { ethers } from 'ethers'

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
    wallet: ethers.Wallet
    attestationsDone: string[] = []

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus()
        this.keys = keys
        this.boundProcessAttestation = this.processAttestation.bind(this)
        this.wallet = new ethers.Wallet(this.keys.ethereum.privateKey)

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

            const comittee = await this.getCommittee()
            this.checkIsInCommittee(comittee, this.keys.substrate.accountId)

            if (!this.isInCurrentCommittee) {
                logger.debug(
                    'Not in current committee, not submitting attestation'
                )
                return
            }

            const attesterEvents = events
                .toHuman()
                .filter((event) => event.event.section == 'attesters')

            // Update metrics
            this.prometheus.eventsTotal.inc(events.length)
            this.prometheus.attestationsInQueue.set(this.q.length())

            logger.debug(this.q, 'Queue content')

            await this.processAttestationEvents(attesterEvents)
        })
    }

    private async processAttestationEvents(events: any) {
        // Loop through the Vec<EventRecord>
        await Promise.all(
            events.map(async (record) => {
                // Extract the phase, event and the event types
                const { event } = record

                if (record.event.section != 'attesters') {
                    return
                }

                logger.debug(
                    {
                        event: event.method,
                        data: event.data,
                    },
                    'Received attesters event'
                )
                this.prometheus.attestationsEvents.inc({ method: event.method })

                switch (event.method) {
                    case 'NewAttestationMessageHash': {
                        const [targetId, messageHash, executionVendor] =
                            event.data

                        this.prometheus.attestionsReceived.inc({
                            targetId: targetId,
                        })

                        if (!this.isValidAttestation(messageHash, targetId))
                            return

                        this.prometheus.attestionsReceivedValid.inc({
                            targetId: targetId,
                        })

                        if (executionVendor.toString() == 'Substrate') {
                            logger.warn('Substrate not implemented')
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

                        messageHashes.forEach(async (messageHash) => {
                            this.prometheus.attestionsReceived.inc({
                                targetId: targetId,
                            })

                            // If attestation is not valid, skip
                            if (
                                !this.isValidAttestation(
                                    messageHash[1],
                                    targetId
                                )
                            )
                                return

                            this.prometheus.attestionsReceivedValid.inc({
                                targetId: targetId,
                            })

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
                        break
                    }

                    default: {
                        logger.debug(
                            {
                                event: event,
                                section: event.section,
                            },
                            'Received unhandled event'
                        )
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

    private isValidAttestation(messageHash: string, targetId: string) {
        return (
            !this.isAttestationDone(messageHash) &&
            this.isTargetAllowed(targetId)
        )
    }

    private isAttestationDone(messageHash: string) {
        return this.attestationsDone.includes(messageHash)
    }

    private isTargetAllowed(targetId: string) {
        return this.config.targetsAllowed.includes(targetId)
    }

    private async processAttestation(data: any) {
        if (!this.isValidAttestation(data.messageHash, data.targetId)) return
        if (!this.isInCurrentCommittee) return

        this.attestationsDone.push(data.messageHash)
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
        const { signature, tx } = await this.generateAttestationTx(
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
            if (errorName == 'attesters::AttestationDoubleSignAttempt') {
                this.attestationsDone.push(messageHash)
                logger.warn(
                    {
                        messageHash: messageHash,
                        targetId: targetId,
                    },
                    'Attestation already submitted'
                )
            }
            return
        }

        this.prometheus.attestationSubmitted.inc({
            targetId: targetId,
            executionVendor: executionVendor,
        })

        logger.info(
            {
                executionVendor: executionVendor,
                targetId: targetId,
                messageHash: messageHash,
                block: result,
            },
            'Attestation submitted'
        )
    }

    private async generateAttestationTx(messageHash: string, targetId: string) {
        // TODO: move to class property
        const signature = await this.wallet.signMessage(
            ethers.getBytes(messageHash)
        )

        const tx = this.circuit.client.tx.attesters.submitAttestation(
            messageHash,
            signature.toString(),
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
