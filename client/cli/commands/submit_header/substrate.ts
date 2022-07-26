import { TypeRegistry, createType } from "@polkadot/types"
import {decodeJustification} from "../../utils/decoder";

export const submitParachainHeader = async () => {

}

export const fetchRelaychainArgs = async (circuitApi: any, targetApi: any, gatewayId: string, blockHeight: number) => {
    // get full block block to add
    const blockHeader = await targetApi.rpc.chain.getBlock(
        await targetApi.rpc.chain.getBlockHash(blockHeight)
    );
    // To understand the proof: https://github.com/paritytech/substrate/issues/7115
    const finalityProof = await targetApi.rpc.grandpa.proveFinality(blockHeight);
    let {latestBlockHash, justification, range} = decodeFinalityProof(finalityProof)
    // signed header is the latest one of the blocks authority set
    const signedHeader = range.pop()
    // we reverse the range for confirming
    range = range.reverse();
    // add the selected header to the range as the last element
    range.push(blockHeader.block.header);
    let rangeType = circuitApi.createType("Vec<Header>", range);
    console.log(`Submitting #${blockHeight} until ${signedHeader.number}`)
    const relaychainHeaderData = circuitApi.createType("RelaychainHeaderData<Header>", {
        signed_header: signedHeader,
        range: rangeType,
        justification: decodeJustification(justification)
    })
    gatewayId = circuitApi.createType("ChainId", gatewayId);
    return {
        gateway_id: gatewayId,
        header_data: relaychainHeaderData
    }
}

const finalityProof = { proof: "(Header::Hash, Vec<u8>, Vec<Header>)" }
export const decodeFinalityProof = (data: any) => {
    const registry = new TypeRegistry()
    registry.register(finalityProof);

    const res = createType(registry, finalityProof.proof, data.toJSON()) // toJSON works, toHEX() not
    return {latestBlockHash: res[0], justification: res[1], range: res[2]}
}