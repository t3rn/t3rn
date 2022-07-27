import { TypeRegistry, createType } from "@polkadot/types"
import {decodeJustification} from "../../utils/decoder";

export const submitParachainHeader = async () => {

}

export const submitRelaychainHeaders = async (circuitApi: any, targetApi: any, gatewayId: string) => {
    const from = (await fetchGatewayHeight(gatewayId, circuitApi)) + 1;
    console.log("Latest on Gateway:", from - 1);
    const to = await fetchCurrentHeight(targetApi);
    console.log("Latest on Target: ", to.toNumber());
    return generateBatchProof(circuitApi, targetApi, gatewayId, from, to)
}

const generateBatchProof = async (circuitApi: any, targetApi: any, gatewayId: string, from: number, to: number) => {
    let transactionArguments: any[] = [];
    while(from < to) {
        // console.log("From:", from)
        const finalityProof = await targetApi.rpc.grandpa.proveFinality(from)
        let {latestBlockHash, justification, headers} = decodeFinalityProof(finalityProof)
        let signed_header = headers.pop()
        headers.reverse() // we need to submit backwards
        headers.push(await getHeader(targetApi, from)) // we need to fetch the target header seperatly
        let range = circuitApi.createType("Vec<Header>", headers)
        console.log("Batch:")
        console.log(`Range: From #${range[range.length - 1].number.toNumber()} to #${range[0].number.toNumber()}`)
        console.log("Signed Header:", signed_header.toHuman())
        console.log("Justification:", decodeJustification(justification).toHuman())
        console.log("__________________________________________________________________")
        const relaychainHeaderData = circuitApi.createType("RelaychainHeaderData<Header>", {
            signed_header,
            range,
            justification: decodeJustification(justification)
        })
        transactionArguments.push(relaychainHeaderData)
        from = parseInt(signed_header.number.toJSON()) + 1
    }
    return transactionArguments;
}

const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
export const decodeFinalityProof = (data: any) => {
    const registry = new TypeRegistry()
    registry.register(finalityProof);

    const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
    return {latestBlockHash: res[0], justification: res[1], headers: res[2]}
}

// ToDo this should be replaced for portal RPC call once #380 is closed
const fetchGatewayHeight = async (gatewayId: any, circuitApi: any) =>  {
    const hash = await circuitApi.query.rococoBridge.bestFinalizedMap(gatewayId);
    const height = await circuitApi.query.rococoBridge.multiImportedHeaders(gatewayId, hash.toJSON());
    if (height.toJSON()) {
        return height.toJSON().number
    } else {
        console.log("Gateway not Registered!")
        process.exit(1)
    }
}

const fetchCurrentHeight = async (targetApi: any) => {
    const header = await targetApi.rpc.chain.getHeader(
        await targetApi.rpc.chain.getFinalizedHead()
    );

    return header.number
}

const getHeader = async (targetApi: any, height: number) => {
    return targetApi.rpc.chain.getHeader(
        await targetApi.rpc.chain.getBlockHash(height)
    )
}