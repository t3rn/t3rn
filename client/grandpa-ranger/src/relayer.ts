import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { BlockHash, Header } from "@polkadot/types/interfaces"
import RelaychainListener from "./listeners/relaychain"
import ParachainListener from "./listeners/parachain"
import createDebug from "debug"
import types from "./types.json"
import { EventEmitter } from "events"
import { fetchNonce } from "./util"

export default class Relayer extends EventEmitter {
  static debug = createDebug("relayer")
  api: ApiPromise
  signer: any

  async setup(url: string) {
    this.api = await ApiPromise.create({
      provider: new WsProvider(url),
      types: types as any,
    })

    const keyring = new Keyring({ type: "sr25519" })

    this.signer =
      process.env.CIRCUIT_KEY === undefined
        ? keyring.addFromUri("//Alice")
        : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

    Relayer.debug("Relayer Setup complete")
  }

  async submitFinalityProof(
    gatewayId: string,
    justification: any,
    anchorHeader: Header,
    anchorIndex: number
  ) {
    const nonce = await fetchNonce(this.api, this.signer.address)
    Relayer.debug("submitFinalityProof nonce", nonce.toString())

    await this.api.tx.multiFinalityVerifierDefault
      .submitFinalityProof(anchorHeader, justification, gatewayId)
      .signAndSend(this.signer, { nonce }, async result => {
        if (result.isError) {
          Relayer.debug("FinalityProofSubmitted failed")
        } else if (result.isInBlock) {
          this.emit("FinalityProofSubmitted", {
            gatewayId,
            anchorHash: anchorHeader.hash,
            anchorIndex,
          })
        }
      })
  }

  async submitParachainHeaders(
    params: {
      gatewayId: string
      blockHash: BlockHash
      proof: any
      anchorNumber: number
    }[]
  ): Promise<
    {
      gatewayId: string
      blockHash: BlockHash
      proof: any
      anchorNumber: number
    }[]
  > {
    const nonce = await fetchNonce(this.api, this.signer.address)
    Relayer.debug("submitParachainHeaders nonce", nonce.toString())

    return new Promise(async (resolve, reject) => {
      await this.api.tx.utility
        .batch(
          params.map(({ gatewayId, blockHash, proof }) =>
            this.api.tx.circuitPortal.submitParachainHeader(
              blockHash,
              gatewayId,
              proof
            )
          )
        )
        .signAndSend(this.signer, { nonce }, result => {
          if (result.isError) {
            Relayer.debug(
              "batch submitParachainHeader failed",
              JSON.stringify(result)
            )
            reject(Error(JSON.stringify(result)))
          } else if (result.isInBlock) {
            Relayer.debug(
              `submitted parachain headers for ${params.map(p => p.gatewayId)}`
            )
            resolve(params)
          }
        })
    })
  }

  async submitHeaderRanges(
    params: {
      gateway: ParachainListener | RelaychainListener
      gatewayId: string
      anchorNumber: number
    }[]
  ) {
    const nonce = await fetchNonce(this.api, this.signer.address)
    Relayer.debug("submitHeaderRanges nonce", nonce.toString())

    await this.api.tx.utility
      .batch(
        params
          .map(({ gatewayId, gateway, anchorNumber }) => {
            const i = gateway.headerListener.headers.findIndex(
              (header: Header) => header.number.toNumber() === anchorNumber
            )
            if (i === -1) {
              Relayer.debug(
                `skipping submitHeaderRange for ${gatewayId} due to missing anchor ${anchorNumber}`
              )
              return null
            } else {
              const range = gateway.headerListener.headers
                .slice(0, i + 1)
                .reverse()
              const anchorHeader = range.shift()

              return this.api.tx.multiFinalityVerifierDefault.submitHeaderRange(
                gatewayId,
                range,
                anchorHeader.hash
              )
            }
          })
          .filter(Boolean)
      )
      .signAndSend(this.signer, { nonce }, async result => {
        if (result.isError) {
          Relayer.debug(
            "batch submitHeaderRange failed",
            JSON.stringify(result)
          )
        } else if (result.status.isFinalized) {
          this.emit("SubmittedHeaderRanges", params)
        }
      })
  }
}
