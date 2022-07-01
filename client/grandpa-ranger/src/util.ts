import { TypeRegistry, createType } from "@polkadot/types"
import { Header } from "@polkadot/types/interfaces"
import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"

const registry = new TypeRegistry()
const justification = { type: 'GrandpaJustification<Header>' }
export const decodeJustificationTarget = (data: any) => {
    const justification = decodeJustification(data)

    return justification.commit.targetNumber.toNumber()
}

// gets events of a specific block, and checks if authoritySetUpdate is included.
// it would be more efficient to listen for the NewAuthorities event directly, but I'm unable to decode the block number currently.
export const containsAuthoritySetUpdate= async (api: any, blockNumber: number) => {
    const hash = await api.rpc.chain.getBlockHash(blockNumber);
    const notifications = await api.query.system.events.at(hash);
    return notifications.some(noti => noti.event.method === "NewAuthorities")
}

export const decodeJustification = (data: any) => {
    registry.register(justification);
    return createType(registry, justification.type as any, data)
}

const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
export const decodeFinalityProof = (data: any) => {
    registry.register(finalityProof);

    const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
    return {blockHash: res[0], justification: res[1]}
}

// not ideal, but I'm unable to use the "NewAuthorites" event, because I'm unable to figure out in which block the event was emitted
export const decodeAuthoritySet = (data: any) => {
    const justification = decodeJustification((data))
    return justification.commit.precommits.map(entry => entry.id.toHex()).sort();
}

export const fetchGatewayHeight = (gatewayId: any, circuit: any) => {
    return 798779
}


export function formatEvents(
    events: { event: { section: string; method: string; data: any } }[]
): string[] {
    return events.map(
        ({ event: { data, method, section } }) =>
            `${section}.${method} ${data.toString()}`
    )
}

export function decodeHeaderNumber(data: string) {
    // removes the Vec Decoding, bit hacky
    if (data.slice(0, 6) === "0xe902") {
        data = "0x" + data.split("e902")[1]
    }

    const typeObject = { type: "Block::Header" }
    registry.register(typeObject)
    const res: any = createType(registry, typeObject.type, data)
    return res.number.toNumber()
}

export async function fetchNonce(
    api: ApiPromise,
    address: string
): Promise<BN> {
    return api.rpc.system.accountNextIndex(address)
}

export async function fetchMissingHeaders(
    api: ApiPromise,
    headers: (Header | number)[],
    until?: number
): Promise<Header[]> {
    let _headers
    if (until) {
        _headers = Array.from(headers)
        let tail =
            typeof headers[headers.length - 1] === "number"
                ? (headers[headers.length - 1] as number)
                : (headers[headers.length - 1] as Header).number.toNumber()
        while (++tail <= until) _headers.push(tail)
    } else {
        _headers = headers
    }
    return Promise.all(
        _headers.map(async h => {
            if (typeof h === "number") {
                const blockHash = await api.rpc.chain.getBlockHash(h)
                return api.rpc.chain.getHeader(blockHash)
            } else {
                return h
            }
        })
    )
}