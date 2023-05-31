// eslint-disable-next-line @typescript-eslint/no-var-requires
require("dotenv").config()
import { Connection } from "./connection"
import { cryptoWaitReady } from "@t3rn/sdk"
import { Prometheus } from "./prometheus"
import fs from "fs"
import pino from "pino"
import ethUtil from "ethereumjs-util"
import { hexToU8a } from "@polkadot/util"

// Determine if pretty printing is enabled based on the PROFILE environment variable
const isPrettyPrintEnabled =
    process.env.PROFILE === "local" || process.env.LOG_PRETTY === "true"

const { stderr } = process
// Create a writable stream that discards the output
const NullWritable = fs.createWriteStream("/dev/null")

// Redirect stdout to the NullWritable stream
// stdout.write = NullWritable.write.bind(NullWritable)
stderr.write = NullWritable.write.bind(NullWritable)

const loggerConfig = {
    level: process.env.LOG_LEVEL || "info",
    formatters: {
        level: (label) => {
            return { level: label }
        },
    },
    base: undefined,
    stream: process.stdout,
    transport: isPrettyPrintEnabled
        ? {
              target: "pino-pretty",
          }
        : undefined,
}

const logger = pino(loggerConfig)

// Apply the pino-pretty formatter if pretty printing is enabled

class Attester {
    circuit: Connection
    target: Connection
    config: any
    prometheus: Prometheus

    constructor(config: any, keys: any) {
        this.config = config
        this.prometheus = new Prometheus(this.config.targetGatewayId)
        // this.prometheus.rangeInterval.inc({target: this.target}, this.config.rangeInterval)
        // this.prometheus.nextSubmission.set({target: this.target}, Date.now() + this.config.rangeInterval * 1000)
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
            true,
            this.prometheus,
            this.config.targetGatewayId,
            this.config.circuitSigner
        )
        this.circuit.connect()
    }

    async listenEvents() {
        // Subscribe to the NewAttestationMessageHash event
        this.circuit.sdk?.client.query.system.events(async (events) => {
            logger.info(`Received ${events.length} events`)
            // Loop through the Vec<EventRecord>
            await Promise.all(
                events.map(async (record) => {
                    // Extract the phase, event and the event types
                    const { event } = record
                    logger.debug(event.toHuman())

                    if (event.section == "attesters") {
                        logger.info(
                            `${event.section}:${event.method}:: (phase=${record.phase})`
                        )

                        switch (event.method) {
                            case "NewAttestationMessageHash": {
                                const [targetId, messageHash, executionVendor] =
                                    event.data
                                logger.info(
                                    `Received the attestation message hash request to sign`,
                                    {
                                        targetId: targetId.toString(),
                                        messageHash: messageHash.toHex(),
                                        executionVendor:
                                            executionVendor.toString(),
                                    }
                                )
                                // Submit the attestation for the given target ID for the given message hash for each attester's key in the keys.json file
                                if (executionVendor.toString() == "Substrate") {
                                    console.warn("Substrate unhandled yet")
                                } else if (
                                    executionVendor.toString() == "Ed25519"
                                ) {
                                    console.warn("Ed25519 unhandled yet")
                                } else if (
                                    executionVendor.toString() == "EVM"
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

                                    logger.info("Executed", {
                                        executionVendor:
                                            executionVendor.toString(),
                                        targetId: targetId.toString(),
                                        messageHash: messageHash.toHex(),
                                        hash: tx.hash.toHex(),
                                    })
                                }

                                break
                            }
                            case "NewTargetProposed": {
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
    let config: any
    if (process.env.PROFILE === "prod") {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require("../config/prod.ts").default
    } else if (process.env.PROFILE === "roco") {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require("../config/roco.ts").default
    } else {
        // eslint-disable-next-line @typescript-eslint/no-var-requires
        config = require("../config/local.ts").default
    }
    const keys = {
        ethereum: {
            privateKey: process.env.PRIVATE_KEY_ETH,
        },
        substrate: {
            privateKey: process.env.PRIVATE_KEY_SUBSTRATE,
        },
        btc: {
            privateKey: process.env.PRIVATE_KEY_BTC,
        },
    }
    const attester = new Attester(config, keys)
    logger.info(`Starting attester`)
    await attester.start()
})()
