import { TypeRegistry, createType } from "@polkadot/types"
import {decodeJustification} from "../../utils/decoder";

export const submitParachainHeader = async () => {

}

export const fetchRelaychainArgs = async (circuitApi: any, targetApi: any, gatewayId: string, blockNumber: number) => {
    console.log(blockNumber)
    const finalityProof = await targetApi.rpc.grandpa.proveFinality(blockNumber);
    const {blockHash, justification} = decodeFinalityProof(finalityProof, circuitApi)
    const header = await targetApi.rpc.chain.getHeader(blockHash);
    const relaychainHeaderData = circuitApi.createType("RelaychainHeaderData<Header>", {
        header,
        justification: decodeJustification(justification)
    })
    gatewayId = circuitApi.createType("ChainId", gatewayId);
    return {gatewayId, relaychainHeaderData}
}

const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
export const decodeFinalityProof = (data: any, circuitApi: any) => {
    const registry = new TypeRegistry()
    registry.register(finalityProof);

    const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
    return {blockHash: res[0], justification: res[1]}
}