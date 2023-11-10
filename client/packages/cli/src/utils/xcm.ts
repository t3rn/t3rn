import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import { SignerOptions } from "@polkadot/api/types/submittable"
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'

export async function signAndSendXcm(tx: SubmittableExtrinsic, api: ApiPromise, signer: any) {
      await  tx.signAndSend(signer, async ({status, events}) => {
            if (status.isInBlock || status.isFinalized) {
                events
                    // find/filter for failed events
                    .filter(({event}) =>
                        api.events.system.ExtrinsicFailed.is(event),
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(
                        ({
                             event: {
                                 data: [error, info],
                             },
                         }) => {
                            if (error.isModule) {
                                // for module errors, we have the section indexed, lookup
                                const decoded = api.registry.findMetaError(
                                    error.asModule,
                                )
                                const {docs, method, section} = decoded

                                console.log(`${section}.${method}: ${docs.join(' ')}`)
                            } else {
                                // Other, CannotLookup, BadOrigin, no extra info
                                console.log(error.toString())
                            }
                        },
                    )
            }
        })
    return
}