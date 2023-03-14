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

        const relaychainHeaderData = circuit.createType("GrandpaHeaderData<Header>", {
            signed_header,
            range,
            justification: Encodings.Substrate.Decoders.justificationDecode(justification)
        })

        //push to transaction queue
        transactionArguments.push({gatewayId: circuit.createType("ChainId", gatewayId), data: relaychainHeaderData})
        from = parseInt(signed_header.number.toJSON()) + 1
    }

    logger.debug(logMsg)
    return transactionArguments;
}

// const generateParachainProof = async (circuit: any, target: any, gatewayData: any, logger: any) => {
//     const latestRelayChainHeader = await fetchBestFinalizedHash(circuit, gatewayData.registrationData.parachain.relayChainId)
//     const parachainHeader: any = await fetchLatestPossibleParachainHeader(
//         gatewayData.relaychainRpc,
//         latestRelayChainHeader.toJSON(),
//         gatewayData.registrationData.parachain.id
//     )
//     const decodedParachainHeader = Encodings.Substrate.Decoders.headerDecode(parachainHeader.toJSON())
//     const parachainHeightCircuit = await fetchGatewayHeight(gatewayData.id, circuit)
//     let logMsg = {
//         type: "PARACHAIN",
//         gatewayId: gatewayData.id,
//         latesteCircuit: parachainHeightCircuit,
//         latestTarget: decodedParachainHeader.number.toNumber(),
//     };
//
//     let headers = await collectHeaderRange(target, parachainHeightCircuit + 1, decodedParachainHeader.number.toNumber() - 1) // the new highest header is in the proof
//
//     const proof = await getStorageProof(gatewayData.relaychainRpc, latestRelayChainHeader.toJSON(), gatewayData.registrationData.parachain.id)
//
//     logMsg["targetFrom"] = parachainHeightCircuit + 1;
//     logMsg["targetTo"] = decodedParachainHeader.number.toNumber() - 1;
//
//     const parachainHeaderData = circuit.createType("ParachainHeaderData<Header>", {
//         relay_block_hash: latestRelayChainHeader.toJSON(),
//         range: headers,
//         proof: {
//             trieNodes: proof.toJSON().proof
//         }
//     })
//     logger.debug(logMsg)
//     return [{gatewayId: circuit.createType("ChainId", gatewayData.id), data: parachainHeaderData}]
// }

// const collectHeaderRange = async (target: any, from: number, to: number) => {
//     let headers: any[] = [];
//     while(from <= to) {
//         headers.push(
//             (await target.rpc.chain.getHeader(
//                 await target.rpc.chain.getBlockHash(from)
//             )).toJSON()
//         )
//         from += 1
//     }
//     return headers;
// }

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

// //for registrations we want to get the justification cotaining the latest authoritySetUpdate, as we can be sure that all authorties are included.
// const fetchHeaders = async (gatewayData: any, page: number) => {
//     return axios.post(gatewayData.subscan + '/api/scan/blocks', {
//             row: 100,
//             page,
//         },
//         {
//             headers: {
//                 'content-type': 'text/json'
//             }
//         }
//     )
//     .then(function (response) {
//         console.log(response.data.data)
//         // return response.data.data.events.map(entry => entry.block_num)
//     })
// }