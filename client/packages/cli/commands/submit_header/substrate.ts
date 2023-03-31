import { Encodings } from "@t3rn/sdk";
import {fetchBestFinalizedHash, fetchLatestPossibleParachainHeader, getStorageProof} from "../../utils/substrate"
const axios = require('axios').default;

export const submitRelaychainHeaders = async (circuit: any, target: any, gatewayId: string, logger: any) => {
    const from = (await fetchGatewayHeight(gatewayId, circuit)) + 1;
    const to = await fetchCurrentHeight(target);
    return generateBatchProof(circuit, target, gatewayId, from, to, logger)
}

const generateBatchProof = async (circuit: any, target: any, gatewayId: string, from: number, to: number, logger: any) => {
    let batches: any[] = []
    let logMsg = {
        type: "RELAYCHAIN",
        gatewayId,
        latesteCircuit: from,
        latestTarget: to,
        batches,
    };

    let transactionArguments: any[] = [];
    while(from < to) {
        // get finalityProof element of epoch that contains block #from
        const finalityProof = await target.rpc.grandpa.proveFinality(from)
        // decode finality proof
        let { justification, headers } = Encodings.Substrate.Decoders.finalityProofDecode(finalityProof)
        let signed_header = headers.pop()

        // query from header again, as its not part of the proof, and concat
        headers = [await getHeader(target, from), ...headers]
        let range = circuit.createType("Vec<Header>", headers)
        logMsg.batches.push(
            {
                targetFrom: range[0].number.toNumber(),
                targetTo: range[range.length - 1].number.toNumber(),
            }
        )

        justification = Encodings.Substrate.Decoders.justificationDecode(justification);

        //push to transaction queue
        transactionArguments.push({gatewayId: circuit.createType("ChainId", gatewayId), signed_header, range, justification})
        from = parseInt(signed_header.number.toJSON()) + 1
    }

    logger.debug(logMsg)
    return transactionArguments;
}

// ToDo this should be replaced for portal RPC call once #380 is closed
const fetchGatewayHeight = async (gatewayId: any, circuit: any) =>  {
    const hash = await circuit.query.rococoBridge.bestFinalizedHash();
    const height = await circuit.query.rococoBridge.importedHeaders(hash.toJSON());
    if (height.toJSON()) {
        return height.toJSON().number
    } else {
        console.log("Gateway not Registered!")
        process.exit(1)
    }
}

const fetchCurrentHeight = async (target: any) => {
    const header = await target.rpc.chain.getHeader(
        await target.rpc.chain.getFinalizedHead()
    );

    return header.number
}

const getHeader = async (target: any, height: number) => {
    return (await target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(height)
    )).toJSON()
}