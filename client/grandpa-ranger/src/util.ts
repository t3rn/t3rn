import { TypeRegistry, createType } from "@polkadot/types"
import { Header } from "@polkadot/types/interfaces"
import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"

const registry = new TypeRegistry()
const type = { type: "GrandpaJustification<Header>" }
export const decodeJustification = (data: any) => {
  registry.register(type)
  const res = createType(registry, type.type as any, data)

  return res.commit.targetNumber.toNumber()
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
