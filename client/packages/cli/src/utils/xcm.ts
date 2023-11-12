import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import { SignerOptions } from "@polkadot/api/types/submittable"
import { ApiPromise, WsProvider, Keyring } from '@t3rn/sdk'

export async function signAndSend(tx: SubmittableExtrinsic, api: ApiPromise, signer: any) {
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
export async function signAndSendSudo(tx: SubmittableExtrinsic, api: ApiPromise, keyring: Keyring) {
    const adminId = await api.query.sudo.key()
    const adminPair = keyring.getPair(adminId.toString())
    await  api.tx.sudo.sudo(tx).signAndSend(adminPair, async ({status, events}) => {
        if (status.isInBlock || status.isFinalized) {
            events
                // We know this tx should result in `Sudid` event.
                .filter(({event}) => api.events.sudo.Sudid.is(event))
                // We know that `Sudid` returns just a `Result`
                .forEach(
                    ({
                         event: {
                             data: [result],
                         },
                     }) => {
                        // Now we look to see if the extrinsic was actually successful or not...
                        if (result.isError) {
                            const error = result.asError
                            if (error.isModule) {
                                // for module errors, we have the section indexed, lookup
                                const decoded = api.registry.findMetaError(error.asModule)
                                const {docs, name, section} = decoded

                                console.log(`${section}.${name}: ${docs.join(' ')}`)
                            } else {
                                // Other, CannotLookup, BadOrigin, no extra info
                                console.log(error.toString())
                            }
                        }
                    },
                )
            create()
        }
    })
    return
}
