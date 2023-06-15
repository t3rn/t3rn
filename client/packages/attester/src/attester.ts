import { Connection } from './connection'
import { cryptoWaitReady } from '@t3rn/sdk'
import { logger } from './logging'
import { Prometheus } from './prometheus'
import * as ethUtil from 'ethereumjs-util'
import { Mutex } from 'async-mutex'
// ts-ignore
import { hexToU8a } from '@polkadot/util'

export class Attester {
    circuit: Connection
    config: any
    prometheus: Prometheus
    keys: any
    mutex: Mutex
    isInCurrentCommittee = false

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

            const attesterEvents = await events.filter(
                async (event) => (await event.section) == 'attesters'
            )

            this.prometheus.attestationsEvents.inc(attesterEvents.length)

            // Loop through the Vec<EventRecord>
            await Promise.all(
                attesterEvents.map(async (record) => {
                    // Before any attestation we need to check if we are in current committee
                    if (!this.isInCurrentCommittee) {
                        return
                    }

                    // Extract the phase, event and the event types
                    const { event } = record

                    switch (event.method) {
                        case 'NewAttestationMessageHash': {
                            const [targetId, messageHash, executionVendor] =
                                event.data

                            // Submit the attestation for the given target ID for the given message hash for each attester's key in the keys.json file
                            if (executionVendor.toString() == 'Substrate') {
                                logger.warn('Substrate not implemented')
                            } else if (
                                executionVendor.toString() == 'Ed25519'
                            ) {
                                logger.warn('Ed25519 not implemented')
                            } else if (executionVendor.toString() == 'EVM') {
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

                            // Attest all pending attestations
                            messageHashes.forEach(async (messageHash) => {
                                await this.submitAttestationEVM(
                                    messageHash[1],
                                    targetId,
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
            this.prometheus.attestationSubmitError.inc({ error: errorName })
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
